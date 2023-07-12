use std::fmt;

use crate::binding::BindingTable;
use crate::chunk::Chunk;
use crate::op::Op;
use crate::parser::parse;
use crate::syntax::{Expr, Program};
use crate::value::Value;

#[derive(Debug)]
struct Capture {
    name: String,
    is_local: bool,
    index: usize,
}

struct CaptureTable {
    captures: Vec<Capture>,
}

impl CaptureTable {
    fn new() -> Self {
        Self {
            captures: Vec::new(),
        }
    }

    fn add_local_capture(&mut self, name: &str, scope: usize) -> usize {
        let name = name.to_owned();
        let is_local = true;
        let index = scope;
        let capture = Capture {
            name,
            is_local,
            index,
        };
        self.captures.push(capture);
        self.captures.len() - 1
    }

    fn add_capture_capture(&mut self, name: &str, outer_index: usize) -> usize {
        let name = name.to_owned();
        let is_local = false;
        let index = outer_index;
        let capture = Capture {
            name,
            is_local,
            index,
        };
        self.captures.push(capture);
        self.captures.len() - 1
    }

    pub fn lookup(&self, lookup_name: &str) -> Option<usize> {
        self.captures
            .iter()
            .enumerate()
            .rev()
            .find(|(_, c)| c.name == lookup_name)
            .map(|(i, _)| i)
    }
}

impl fmt::Debug for CaptureTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for c in self.captures.iter() {
            let kind = if c.is_local { "local" } else { "capture" };
            write!(f, "{}:{}#{} ", c.name, kind, c.index)?;
        }
        write!(f, "}}")
    }
}

struct Frame {
    stack_top: usize,
    locals: BindingTable,
    captures: CaptureTable,
}

struct CompilerState {
    stack_top: usize,
    save_stack: Vec<usize>,
    locals: BindingTable,
    call_stack: Vec<Frame>,
}

impl CompilerState {
    fn new() -> Self {
        Self {
            stack_top: 0,
            save_stack: Vec::new(),
            locals: BindingTable::new(),
            call_stack: Vec::new(),
        }
    }

    fn push(&mut self) {
        self.stack_top += 1;
    }

    fn pop(&mut self) {
        self.stack_top -= 1;
    }

    fn save_stack(&mut self) {
        self.save_stack.push(self.stack_top);
    }

    fn restore_stack(&mut self) {
        self.stack_top = self.save_stack.pop().expect("save stack underflow");
    }

    fn begin_scope(&mut self, name: &str) {
        self.locals.push(name, self.stack_top - 1);
    }

    fn end_scope(&mut self) {
        self.locals.pop()
    }

    fn begin_proc(&mut self, name: &str, var: &str) {
        let stack_top = std::mem::replace(&mut self.stack_top, 0);
        let locals = std::mem::replace(&mut self.locals, BindingTable::new());
        let frame = Frame {
            stack_top,
            locals,
            captures: CaptureTable::new(),
        };
        self.call_stack.push(frame);

        // simulate pushing proc object and argument
        self.push();
        self.begin_scope(name);
        self.push();
        self.begin_scope(var);
    }

    fn end_proc(&mut self) -> CaptureTable {
        let frame = self.call_stack.pop().expect("call stack underflow");
        self.stack_top = frame.stack_top;
        self.locals = frame.locals;
        frame.captures
    }

    fn lookup_local(&mut self, lookup_name: &str) -> Option<usize> {
        self.locals.lookup(lookup_name)
    }

    fn lookup_capture(&mut self, lookup_name: &str) -> Option<usize> {
        let call_depth = self.call_stack.len();
        if call_depth > 0 {
            self.capture(lookup_name, call_depth - 1)
        } else {
            None
        }
    }

    fn capture(&mut self, lookup_name: &str, call_depth: usize) -> Option<usize> {
        if let Some(stack_index) = self.call_stack[call_depth].locals.lookup(lookup_name) {
            let capture_index = self.call_stack[call_depth]
                .captures
                .add_local_capture(lookup_name, stack_index);
            Some(capture_index)
        } else if let Some(capture_index) = self.call_stack[call_depth].captures.lookup(lookup_name)
        {
            Some(capture_index)
        } else if call_depth > 0 {
            self.capture(lookup_name, call_depth - 1)
                .map(|outer_capture_index| {
                    self.call_stack[call_depth]
                        .captures
                        .add_capture_capture(lookup_name, outer_capture_index)
                })
        } else {
            None
        }
    }
}

/// Parses and compiles a given source text into a chunk.
pub fn compile(src: &str) -> Result<Chunk, String> {
    let program = parse(src)?;
    compile_program(&program)
}

fn compile_program(program: &Program) -> Result<Chunk, String> {
    let mut chunk = Chunk::new();
    let mut state = CompilerState::new();
    compile_expr(&program.expr, &mut chunk, &mut state)?;
    Ok(chunk)
}

fn compile_expr(expr: &Expr, chunk: &mut Chunk, state: &mut CompilerState) -> Result<(), String> {
    match expr {
        Expr::Call(proc, arg) => {
            compile_expr(proc, chunk, state)?;
            compile_expr(arg, chunk, state)?;
            state.pop();
            state.pop();
            chunk.emit(Op::Call);
            state.push();
        }

        Expr::Const(x) => {
            let v = Value::Number(*x);
            chunk.emit(Op::PushValue(v));
            state.push();
        }

        Expr::Diff(e1, e2) => {
            compile_expr(e1, chunk, state)?;
            compile_expr(e2, chunk, state)?;
            state.pop();
            state.pop();
            chunk.emit(Op::Diff);
            state.push();
        }

        Expr::If(guard, consq, alt) => {
            compile_expr(guard, chunk, state)?;
            state.pop();
            let branch_to_consq = chunk.emit(Op::JumpTrue(0));

            state.save_stack();

            compile_expr(alt, chunk, state)?;
            let branch_to_end = chunk.emit(Op::Jump(0));

            state.restore_stack();

            let consq_start = chunk.next_address();
            compile_expr(consq, chunk, state)?;
            let if_end = chunk.next_address();
            chunk.patch(branch_to_consq, consq_start);
            chunk.patch(branch_to_end, if_end);
        }

        Expr::IsZero(e) => {
            compile_expr(e, chunk, state)?;
            state.pop();
            chunk.emit(Op::IsZero);
            state.push();
        }

        Expr::Let(var, e1, e2) => {
            compile_expr(e1, chunk, state)?;
            state.begin_scope(var);
            compile_expr(e2, chunk, state)?;
            state.end_scope();
        }

        Expr::LetRec {
            name,
            var,
            proc_body,
            let_body,
        } => {
            let branch_make_proc = chunk.emit(Op::Jump(0));

            let start = chunk.next_address();
            state.begin_proc(name, var);
            compile_expr(proc_body, chunk, state)?;
            chunk.emit(Op::Return);
            let captures = dbg!(state.end_proc());

            let capture_ops: Vec<crate::op::Capture> = captures
                .captures
                .iter()
                .map(|c| {
                    if c.is_local {
                        crate::op::Capture::Local(c.index)
                    } else {
                        crate::op::Capture::Capture(c.index)
                    }
                })
                .collect();
            let make_proc_index = chunk.emit(Op::MakeProc(start, capture_ops));
            state.push();
            chunk.patch(branch_make_proc, make_proc_index);

            state.begin_scope(name);
            compile_expr(let_body, chunk, state)?;
            state.end_scope();
        }

        Expr::Minus(e) => {
            compile_expr(e, chunk, state)?;
            state.pop();
            chunk.emit(Op::Minus);
            state.push();
        }

        Expr::Proc(var, body) => {
            let branch_make_proc = chunk.emit(Op::Jump(0));

            let start = chunk.next_address();
            state.begin_proc("", var);
            compile_expr(body, chunk, state)?;
            chunk.emit(Op::Return);
            let captures = state.end_proc();

            let capture_ops: Vec<crate::op::Capture> = captures
                .captures
                .iter()
                .map(|c| {
                    if c.is_local {
                        crate::op::Capture::Local(c.index)
                    } else {
                        crate::op::Capture::Capture(c.index)
                    }
                })
                .collect();
            let make_proc_index = chunk.emit(Op::MakeProc(start, capture_ops));
            state.push();
            chunk.patch(branch_make_proc, make_proc_index);
        }

        Expr::Var(var) => {
            if let Some(stack_index) = state.lookup_local(var) {
                chunk.emit(Op::PushLocal(stack_index));
                state.push();
            } else if let Some(capture_index) = state.lookup_capture(var) {
                chunk.emit(Op::PushCapture(capture_index));
                state.push();
            } else {
                return Err(format!("undefined name: {}", var));
            }
        }
    }

    Ok(())
}

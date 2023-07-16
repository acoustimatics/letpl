use std::fmt;

use crate::parser::{Expr, Program};
use crate::runtime::{self, Op, Value};

#[derive(Clone)]
struct Binding {
    name: String,
    stack_index: usize,
}

impl Binding {
    fn new(name: &str, stack_index: usize) -> Self {
        let name = name.to_owned();
        Self { name, stack_index }
    }
}

#[derive(Clone)]
struct BindingTable {
    bindings: Vec<Binding>,
}

impl BindingTable {
    fn new() -> Self {
        let bindings = Vec::new();
        Self { bindings }
    }

    fn push(&mut self, name: &str, stack_index: usize) {
        let binding = Binding::new(name, stack_index);
        self.bindings.push(binding);
    }

    fn pop(&mut self) {
        self.bindings.pop().expect("binding table underflow");
    }

    fn lookup(&self, lookup_name: &str) -> Option<usize> {
        self.bindings
            .iter()
            .rev()
            .find(|b| b.name == lookup_name)
            .map(|b| b.stack_index)
    }
}

impl fmt::Debug for BindingTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for binding in self.bindings.iter() {
            write!(f, "{}#{} ", binding.name, binding.stack_index)?;
        }
        write!(f, "}}")
    }
}

fn lookup(bindings: &Option<BindingTable>, lookup_name: &str) -> Option<usize> {
    match bindings {
        Some(bindings) => bindings.lookup(lookup_name),
        None => None,
    }
}

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
    locals: Option<BindingTable>,
    captures: CaptureTable,
}

struct CompilerState {
    stack_top: usize,
    save_stack: Vec<usize>,
    globals: BindingTable,
    locals: Option<BindingTable>,
    call_stack: Vec<Frame>,
}

impl CompilerState {
    fn new() -> Self {
        Self {
            stack_top: 0,
            save_stack: Vec::new(),
            globals: BindingTable::new(),
            locals: None,
            call_stack: Vec::new(),
        }
    }

    fn call_depth(&self) -> usize {
        self.call_stack.len()
    }

    fn current_bindings(&mut self) -> &mut BindingTable {
        match self.locals.as_mut() {
            Some(locals) => locals,
            None => &mut self.globals,
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
        let stack_index = self.stack_top - 1;
        self.current_bindings().push(name, stack_index);
    }

    fn end_scope(&mut self) {
        self.current_bindings().pop()
    }

    fn begin_proc(&mut self, name: &str, var: &str) {
        let stack_top = std::mem::replace(&mut self.stack_top, 0);
        let locals = std::mem::replace(&mut self.locals, Some(BindingTable::new()));
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
        lookup(&self.locals, lookup_name)
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
        let frame = &mut self.call_stack[call_depth];
        if let Some(stack_index) = lookup(&frame.locals, lookup_name) {
            let capture_index = frame.captures.add_local_capture(lookup_name, stack_index);
            Some(capture_index)
        } else if let Some(capture_index) = frame.captures.lookup(lookup_name) {
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

struct Chunk {
    pub ops: Vec<Op>,
}

impl Chunk {
    fn new() -> Self {
        let ops = Vec::new();
        Chunk { ops }
    }

    fn emit(&mut self, op: Op) -> usize {
        self.ops.push(op);
        self.ops.len() - 1
    }

    fn next_address(&self) -> usize {
        self.ops.len()
    }

    fn patch(&mut self, patch_at: usize, target: usize) {
        match &self.ops[patch_at] {
            Op::Jump(_) => {
                self.ops[patch_at] = Op::Jump(target);
            }
            Op::JumpTrue(_) => {
                self.ops[patch_at] = Op::JumpTrue(target);
            }
            _ => (),
        }
    }
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "*** chunk {} ops ***", self.ops.len())?;
        for (address, op) in self.ops.iter().enumerate() {
            writeln!(f, "{}\t{:?}", address, op)?;
        }
        Ok(())
    }
}

#[derive(PartialEq)]
enum ExprPos {
    Operand,
    Tail,
}

pub fn compile(program: &Program) -> Result<Vec<Op>, String> {
    let mut chunk = Chunk::new();
    let mut state = CompilerState::new();
    compile_expr(&program.expr, ExprPos::Tail, &mut chunk, &mut state)?;
    Ok(chunk.ops)
}

fn compile_expr(expr: &Expr, expr_pos: ExprPos, chunk: &mut Chunk, state: &mut CompilerState) -> Result<(), String> {
    match expr {
        Expr::Call(proc, arg) => {
            compile_expr(proc, ExprPos::Operand,  chunk, state)?;
            compile_expr(arg, ExprPos::Operand, chunk, state)?;
            state.pop();
            state.pop();
            if expr_pos == ExprPos::Tail && state.call_depth() > 0 {
                chunk.emit(Op::TailCall);
            } else {
                chunk.emit(Op::Call);
            }
            state.push();
        }

        Expr::Const(x) => {
            let v = Value::Number(*x);
            chunk.emit(Op::PushValue(v));
            state.push();
        }

        Expr::Diff(e1, e2) => {
            compile_expr(e1, ExprPos::Operand, chunk, state)?;
            compile_expr(e2, ExprPos::Operand, chunk, state)?;
            state.pop();
            state.pop();
            chunk.emit(Op::Diff);
            state.push();
        }

        Expr::If(guard, consq, alt) => {
            compile_expr(guard, ExprPos::Operand, chunk, state)?;
            state.pop();
            let branch_to_consq = chunk.emit(Op::JumpTrue(0));

            state.save_stack();

            compile_expr(alt, ExprPos::Tail, chunk, state)?;
            let branch_to_end = chunk.emit(Op::Jump(0));

            state.restore_stack();

            let consq_start = chunk.next_address();
            compile_expr(consq, ExprPos::Tail, chunk, state)?;
            let if_end = chunk.next_address();
            chunk.patch(branch_to_consq, consq_start);
            chunk.patch(branch_to_end, if_end);
        }

        Expr::IsZero(e) => {
            compile_expr(e, ExprPos::Operand, chunk, state)?;
            state.pop();
            chunk.emit(Op::IsZero);
            state.push();
        }

        Expr::Let(var, e1, e2) => {
            compile_expr(e1, ExprPos::Operand, chunk, state)?;
            state.begin_scope(var);
            compile_expr(e2, ExprPos::Tail, chunk, state)?;
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
            compile_expr(proc_body, ExprPos::Tail, chunk, state)?;
            chunk.emit(Op::Return);
            let captures = state.end_proc();

            let capture_ops: Vec<runtime::Capture> = captures
                .captures
                .iter()
                .map(|c| {
                    if c.is_local {
                        runtime::Capture::Local(c.index)
                    } else {
                        runtime::Capture::Capture(c.index)
                    }
                })
                .collect();
            let make_proc_index = chunk.emit(Op::MakeProc(start, capture_ops));
            state.push();
            chunk.patch(branch_make_proc, make_proc_index);

            state.begin_scope(name);
            compile_expr(let_body, ExprPos::Tail, chunk, state)?;
            state.end_scope();
        }

        Expr::Minus(e) => {
            compile_expr(e, ExprPos::Operand, chunk, state)?;
            state.pop();
            chunk.emit(Op::Minus);
            state.push();
        }

        Expr::Print(e) => {
            compile_expr(e, ExprPos::Operand, chunk, state)?;
            chunk.emit(Op::Print);
        }

        Expr::Proc(var, body) => {
            let branch_make_proc = chunk.emit(Op::Jump(0));

            let start = chunk.next_address();
            state.begin_proc("", var);
            compile_expr(body, ExprPos::Tail, chunk, state)?;
            chunk.emit(Op::Return);
            let captures = state.end_proc();

            let capture_ops: Vec<runtime::Capture> = captures
                .captures
                .iter()
                .map(|c| {
                    if c.is_local {
                        runtime::Capture::Local(c.index)
                    } else {
                        runtime::Capture::Capture(c.index)
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
            } else if let Some(stack_index) = state.globals.lookup(var) {
                chunk.emit(Op::PushGlobal(stack_index));
                state.push();
            } else {
                return Err(format!("undefined name: {}", var));
            }
        }
    }

    Ok(())
}

use std::fmt;

use crate::binding::{BindingTable, Scope};
use crate::chunk::{Address, Chunk};
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

    fn add_local_capture(&mut self, name: &str, scope: Scope) -> usize {
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

struct CompilerState {
    chunk: Chunk,
    local: BindingTable,
    outer: Vec<BindingTable>,
    captures: Vec<CaptureTable>,
    depth: usize,
}

impl CompilerState {
    fn new() -> Self {
        let chunk = Chunk::new();
        let fake_local = BindingTable::new();
        let outer = Vec::new();
        let captures = Vec::new();
        let depth = 0;
        Self {
            chunk,
            local: fake_local,
            outer,
            captures,
            depth,
        }
    }

    fn bind(&mut self, name: &str) -> Address {
        let a = self.chunk.emit(Op::Bind);
        self.local.push(name);
        a
    }

    fn unbind(&mut self) -> Result<(), String> {
        self.local.pop()?;
        Ok(())
    }

    fn begin_proc(&mut self) {
        let fake_local = std::mem::replace(&mut self.local, BindingTable::new());
        self.outer.push(fake_local);
        self.captures.push(CaptureTable::new());
        self.depth += 1;
    }

    fn end_proc(&mut self) -> CaptureTable {
        self.local = self.outer.pop().unwrap();
        let captures = self.captures.pop().unwrap();
        self.depth -= 1;
        captures
    }

    fn lookup(&mut self, lookup_name: &str) -> Option<Op> {
        match self.local.lookup(lookup_name) {
            Some(scope) => Some(Op::PushLocal(*scope)),
            None if self.depth > 0 => {
                self.capture(lookup_name, self.depth - 1)
                    .map(|index| Op::PushCapture(index))
            }
            None => None,
        }
    }

    fn capture(&mut self, lookup_name: &str, level: usize) -> Option<usize> {
        if let Some(scope) = self.outer[level].lookup(lookup_name) {
            let index = self.captures[level].add_local_capture(lookup_name, *scope);
            Some(index)
        } else if let Some(index) = self.captures[level].lookup(lookup_name) {
            Some(index)
        } else if level > 0 {
            self.capture(lookup_name, level - 1).map(|outer_index| {
                self.captures[level].add_capture_capture(lookup_name, outer_index)
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
    let mut state = CompilerState::new();
    compile_expr(&program.expr, &mut state)?;
    Ok(state.chunk)
}

fn compile_expr(expr: &Expr, state: &mut CompilerState) -> Result<(), String> {
    match expr {
        Expr::Call(proc, arg) => {
            compile_expr(proc, state)?;
            compile_expr(arg, state)?;
            state.chunk.emit(Op::Call);
        }

        Expr::Const(x) => {
            let v = Value::Number(*x);
            state.chunk.emit(Op::PushValue(v));
        }

        Expr::Diff(e1, e2) => {
            compile_expr(e1, state)?;
            compile_expr(e2, state)?;
            state.chunk.emit(Op::Diff);
        }

        Expr::If(guard, consq, alt) => {
            compile_expr(guard, state)?;
            let branch_to_consq = state.chunk.emit(Op::JumpTrue(0));
            compile_expr(alt, state)?;
            let branch_to_end = state.chunk.emit(Op::Jump(0));
            let consq_start = state.chunk.next_address();
            compile_expr(consq, state)?;
            let if_end = state.chunk.next_address();
            state.chunk.patch(branch_to_consq, consq_start);
            state.chunk.patch(branch_to_end, if_end);
        }

        Expr::IsZero(e) => {
            compile_expr(e, state)?;
            state.chunk.emit(Op::IsZero);
        }

        Expr::Let(var, e1, e2) => {
            compile_expr(e1, state)?;
            state.bind(var);
            compile_expr(e2, state)?;
            state.unbind()?;
        }

        Expr::LetRec {
            name,
            var,
            proc_body,
            let_body,
        } => {
            let branch_make_proc = state.chunk.emit(Op::Jump(0));

            state.begin_proc();
            let start = state.bind(var);
            state.bind(name);
            compile_expr(proc_body, state)?;
            state.chunk.emit(Op::Return);
            let captures = state.end_proc();

            let capture_ops: Vec<crate::op::Capture> = captures.captures
                .iter()
                .map(|c| {
                    if c.is_local {
                        crate::op::Capture::Local(c.index)
                    } else {
                        crate::op::Capture::Capture(c.index)
                    }
                })
                .collect();

            let make_proc_index = state.chunk.emit(Op::MakeProc(start, capture_ops));
            state.chunk.patch(branch_make_proc, make_proc_index);
            state.bind(name);
            compile_expr(let_body, state)?;
            state.unbind()?;
        }

        Expr::Minus(e) => {
            compile_expr(e, state)?;
            state.chunk.emit(Op::Minus);
        }

        Expr::Proc(var, body) => {
            let branch_make_proc = state.chunk.emit(Op::Jump(0));
            state.begin_proc();
            let start = state.bind(var);
            state.chunk.emit(Op::Pop);
            compile_expr(body, state)?;
            state.chunk.emit(Op::Return);
            let captures = state.end_proc();

            let capture_ops: Vec<crate::op::Capture> = captures.captures
                .iter()
                .map(|c| {
                    if c.is_local {
                        crate::op::Capture::Local(c.index)
                    } else {
                        crate::op::Capture::Capture(c.index)
                    }
                })
                .collect();
            let make_proc_index = state.chunk.emit(Op::MakeProc(start, capture_ops));
            state.chunk.patch(branch_make_proc, make_proc_index);
        }

        Expr::Var(var) => {
            match state.lookup(var) {
                Some(op) => state.chunk.emit(op),
                None => return Err(format!("undefined name: {}", var)),
            };
        }
    }

    Ok(())
}

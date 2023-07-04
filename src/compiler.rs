use crate::binding::BindingTable;
use crate::chunk::{Address, Chunk};
use crate::op::Op;
use crate::parser::parse;
use crate::syntax::{Expr, Program};
use crate::value::Value;

struct CompilerState {
    chunk: Chunk,
    bindings: BindingTable,
}

impl CompilerState {
    fn new() -> Self {
        let chunk = Chunk::new();
        let bindings = BindingTable::new();
        Self { chunk, bindings }
    }

    fn bind(&mut self, name: &str) -> Address {
        let a = self.chunk.emit(Op::Bind);
        self.bindings.push(name);
        a
    }

    fn unbind(&mut self) -> Result<(), String> {
        self.chunk.emit(Op::Unbind);
        self.bindings.pop()
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

            let start = state.bind(var);
            state.bind(name);
            compile_expr(proc_body, state)?;
            state.chunk.emit(Op::Return);
            state.bindings.pop()?;
            state.bindings.pop()?;

            let make_proc_index = state.chunk.emit(Op::MakeProc(start));
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
            let start = state.bind(var);
            state.chunk.emit(Op::Pop);
            compile_expr(body, state)?;
            state.chunk.emit(Op::Return);
            state.bindings.pop()?;
            let make_proc_index = state.chunk.emit(Op::MakeProc(start));
            state.chunk.patch(branch_make_proc, make_proc_index);
        }

        Expr::Var(var) => {
            let scope = match state.bindings.lookup(var) {
                Some(depth) => *depth,
                None => return Err(format!("undefined name: {}", var)),
            };
            state.chunk.emit(Op::PushBinding(scope));
        }
    }

    Ok(())
}

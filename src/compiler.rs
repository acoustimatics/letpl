use crate::chunk::Chunk;
use crate::op::Op;
use crate::parser::parse;
use crate::syntax::{Expr, Program};
use crate::value::Value;

/// Parses and compiles a given source text into a chunk.
pub fn compile(src: &str) -> Result<Chunk, String> {
    let program = parse(src)?;
    compile_program(&program)
}

fn compile_program(program: &Program) -> Result<Chunk, String> {
    let mut chunk = Chunk::new();
    compile_expr(&program.expr, &mut chunk)?;
    Ok(chunk)
}

fn compile_expr(expr: &Expr, chunk: &mut Chunk) -> Result<(), String> {
    match expr {
        Expr::Const(x) => {
            let v = Value::Number(*x);
            chunk.emit(Op::PushValue(v));
        }
        Expr::Diff(e1, e2) => {
            compile_expr(e1, chunk)?;
            compile_expr(e2, chunk)?;
            chunk.emit(Op::Diff);
        }
        Expr::If(guard, consq, alt) => {
            compile_expr(guard, chunk)?;
            let branch_to_consq = chunk.emit(Op::JumpTrue(0));
            compile_expr(alt, chunk)?;
            let branch_to_end = chunk.emit(Op::Jump(0));
            let consq_start = chunk.next_index();
            compile_expr(consq, chunk)?;
            let if_end = chunk.next_index();
            chunk.patch(branch_to_consq, consq_start);
            chunk.patch(branch_to_end, if_end);
        }
        Expr::IsZero(e) => {
            compile_expr(e, chunk)?;
            chunk.emit(Op::IsZero);
        }
        Expr::Let(id, e1, e2) => {
            compile_expr(e1, chunk)?;
            chunk.emit(Op::Bind(id.clone()));
            compile_expr(e2, chunk)?;
            chunk.emit(Op::Unbind);
        }
        Expr::Var(var) => {
            chunk.emit(Op::Apply(var.clone()));
        }
        Expr::Proc(var, body) => {
            let branch_make_proc = chunk.emit(Op::Jump(0));
            let proc_index = chunk.next_index();
            compile_expr(body, chunk)?;
            chunk.emit(Op::Return);
            let make_proc_index = chunk.emit(Op::MakeProc(var.clone(), proc_index));
            chunk.patch(branch_make_proc, make_proc_index);
        }
        Expr::Call(proc, arg) => {
            compile_expr(proc, chunk)?;
            compile_expr(arg, chunk)?;
            chunk.emit(Op::Call);
        }
    }
    Ok(())
}

use std::fmt;

use crate::name_analysis::{self, Expr, Program};
use crate::runtime::{self, Op, Value};

#[derive(PartialEq)]
enum ExprPos {
    Operand,
    Tail,
}

#[derive(Copy, Clone, PartialEq)]
enum Scope {
    Global,
    Local,
}

struct Chunk {
    pub ops: Vec<Op>,
}

pub fn compile(program: &Program) -> Result<Vec<Op>, String> {
    let mut chunk = Chunk::new();
    compile_expr(&program.expr, Scope::Global, ExprPos::Tail, &mut chunk)?;
    Ok(chunk.ops)
}

fn compile_expr(
    expr: &Expr,
    scope: Scope,
    expr_pos: ExprPos,
    chunk: &mut Chunk,
) -> Result<(), String> {
    match expr {
        Expr::Capture(i) => {
            chunk.emit(Op::PushCapture(*i));
        }

        Expr::Call(proc, arg) => {
            compile_expr(proc, scope, ExprPos::Operand, chunk)?;
            compile_expr(arg, scope, ExprPos::Operand, chunk)?;
            if scope == Scope::Local && expr_pos == ExprPos::Tail {
                chunk.emit(Op::TailCall);
            } else {
                chunk.emit(Op::Call);
            }
        }

        Expr::Const(x) => {
            let v = Value::Number(*x);
            chunk.emit(Op::PushValue(v));
        }

        Expr::Diff(e1, e2) => {
            compile_expr(e1, scope, ExprPos::Operand, chunk)?;
            compile_expr(e2, scope, ExprPos::Operand, chunk)?;
            chunk.emit(Op::Diff);
        }

        Expr::Global(i) => {
            chunk.emit(Op::PushGlobal(*i));
        }

        Expr::If(guard, consq, alt) => {
            compile_expr(guard, scope, ExprPos::Operand, chunk)?;
            let branch_to_consq = chunk.emit(Op::JumpTrue(0));
            compile_expr(alt, scope, ExprPos::Tail, chunk)?;
            let branch_to_end = chunk.emit(Op::Jump(0));
            let consq_start = chunk.next_address();
            compile_expr(consq, scope, ExprPos::Tail, chunk)?;
            let if_end = chunk.next_address();
            chunk.patch(branch_to_consq, consq_start);
            chunk.patch(branch_to_end, if_end);
        }

        Expr::IsZero(e) => {
            compile_expr(e, scope, ExprPos::Operand, chunk)?;
            chunk.emit(Op::IsZero);
        }

        Expr::Let(rhs, body) => {
            compile_expr(rhs, scope, ExprPos::Operand, chunk)?;
            compile_expr(body, scope, ExprPos::Tail, chunk)?;
        }

        Expr::Local(i) => {
            chunk.emit(Op::PushLocal(*i));
        }

        Expr::Print(e) => {
            compile_expr(e, scope, ExprPos::Operand, chunk)?;
            chunk.emit(Op::Print);
        }

        Expr::Proc(body, captures) => {
            let branch_make_proc = chunk.emit(Op::Jump(0));
            let start = chunk.next_address();
            compile_expr(body, Scope::Local, ExprPos::Tail, chunk)?;
            chunk.emit(Op::Return);
            let captures: Vec<runtime::Capture> = captures
                .iter()
                .map(|c| match c {
                    name_analysis::Cap::Local(i) => runtime::Capture::Local(*i),
                    name_analysis::Cap::Capture(i) => runtime::Capture::Capture(*i),
                })
                .collect();
            let make_proc_index = chunk.emit(Op::MakeProc(start, captures));
            chunk.patch(branch_make_proc, make_proc_index);
        }
    }

    Ok(())
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

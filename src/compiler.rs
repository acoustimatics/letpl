//! A bytecode compiler for letpl.

use std::fmt;

use crate::ast::nameless::{Expr, Program};
use crate::runtime::{Address, Op, Value};

#[derive(Copy, Clone, PartialEq)]
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
        Expr::Assert { line, test, body } => {
            compile_expr(test, scope, ExprPos::Operand, chunk)?;
            chunk.emit(Op::Assert { line: *line });
            compile_expr(body, scope, ExprPos::Tail, chunk)?;
        }

        Expr::Capture(capture_offset) => {
            chunk.emit(Op::PushCapture(*capture_offset));
        }

        Expr::Call { proc, arg } => {
            compile_expr(proc, scope, ExprPos::Operand, chunk)?;
            compile_expr(arg, scope, ExprPos::Operand, chunk)?;
            if scope == Scope::Local && expr_pos == ExprPos::Tail {
                chunk.emit(Op::TailCall);
            } else {
                chunk.emit(Op::Call);
            }
        }

        Expr::LiteralInt(x) => {
            let v = Value::Integer(*x);
            chunk.emit(Op::PushValue(v));
        }

        Expr::Subtract { left, right } => {
            compile_expr(left, scope, ExprPos::Operand, chunk)?;
            compile_expr(right, scope, ExprPos::Operand, chunk)?;
            chunk.emit(Op::Diff);
        }

        Expr::Negate(expr) => {
            compile_expr(expr, scope, ExprPos::Operand, chunk)?;
            chunk.emit(Op::Negate);
        }

        Expr::Global(stack_offset) => {
            chunk.emit(Op::PushGlobal(*stack_offset));
        }

        Expr::If {
            test,
            consequent,
            alternate,
        } => {
            compile_expr(test, scope, ExprPos::Operand, chunk)?;
            let branch_to_consq = chunk.emit(Op::JumpTrue(Address(0)));
            compile_expr(alternate, scope, ExprPos::Tail, chunk)?;
            let branch_to_end = chunk.emit(Op::Jump(Address(0)));
            let consq_start = chunk.next_address();
            compile_expr(consequent, scope, ExprPos::Tail, chunk)?;
            let if_end = chunk.next_address();
            chunk.patch(branch_to_consq, consq_start);
            chunk.patch(branch_to_end, if_end);
        }

        Expr::IsZero(e) => {
            compile_expr(e, scope, ExprPos::Operand, chunk)?;
            chunk.emit(Op::IsZero);
        }

        Expr::Let { expr, body } => {
            compile_expr(expr, scope, ExprPos::Operand, chunk)?;
            compile_expr(body, scope, ExprPos::Tail, chunk)?;
        }

        Expr::LiteralBool(value) => {
            chunk.emit(Op::PushValue(Value::Boolean(*value)));
        }

        Expr::Local(stack_offset) => {
            chunk.emit(Op::PushLocal(*stack_offset));
        }

        Expr::Proc { body, captures } => {
            let branch_make_proc = chunk.emit(Op::Jump(Address(0)));
            let start = chunk.next_address();
            compile_expr(body, Scope::Local, ExprPos::Tail, chunk)?;
            chunk.emit(Op::Return);
            let captures = captures.clone();
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

    fn emit(&mut self, op: Op) -> Address {
        self.ops.push(op);
        Address(self.ops.len() - 1)
    }

    fn next_address(&self) -> Address {
        Address(self.ops.len())
    }

    fn patch(&mut self, patch_at: Address, target: Address) {
        let Address(patch_at) = patch_at;
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
            writeln!(f, "{address}\t{op:?}")?;
        }
        Ok(())
    }
}

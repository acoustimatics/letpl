//! Type checks a letpl program

use crate::ast::{Expr, Program};
use crate::symbol_table::SymbolTable;
use crate::types::LetType;

pub fn let_type_of(program: &Program) -> Result<LetType, String> {
    let mut tenv = SymbolTable::new();
    let_type_of_expr(&program.expr, &mut tenv)
}

fn let_type_of_expr(expr: &Expr, tenv: &mut SymbolTable<LetType>) -> Result<LetType, String> {
    match expr {
        Expr::Assert { guard, body, .. } => {
            let t_guard = let_type_of_expr(guard, tenv)?;
            if !t_guard.is_bool() {
                let msg = format!("assert guard must be type `bool` but got `{t_guard}`");
                return Err(msg);
            }
            let_type_of_expr(body, tenv)
        }

        Expr::Call(proc, arg) => {
            let t_proc = let_type_of_expr(proc, tenv)?;
            let Some((t_param, t_body)) = t_proc.as_proc() else {
                let msg = format!("call expects proc but got `{t_proc}`");
                return Err(msg);
            };
            let t_arg = let_type_of_expr(arg, tenv)?;
            if t_param != &t_arg {
                let msg = format!("call expect `{t_param}` argument but got `{t_arg}`");
                return Err(msg);
            }
            Ok(t_body.clone())
        }

        Expr::Const(_) => Ok(LetType::new_int()),

        Expr::Diff(e1, e2) => {
            let t1 = let_type_of_expr(e1, tenv)?;
            if !t1.is_int() {
                let msg = format!("-() first argument expects `int` but got `{t1}`");
                return Err(msg);
            }
            let t2 = let_type_of_expr(e2, tenv)?;
            if !t2.is_int() {
                let msg = format!("-() second argument expects `int` but got `{t2}`");
                return Err(msg);
            }
            Ok(LetType::new_int())
        }

        Expr::If(cond, consq, alt) => {
            let t_cond = let_type_of_expr(cond, tenv)?;
            if !t_cond.is_bool() {
                let msg = format!("`if` condition expects `bool` but got `{t_cond}`");
                return Err(msg);
            }

            let t_consq = let_type_of_expr(consq, tenv)?;
            let t_alt = let_type_of_expr(alt, tenv)?;
            if t_consq != t_alt {
                let msg = format!(
                    "`if` branches expect matching types but got `{t_consq}` and `{t_alt}`"
                );
                return Err(msg);
            }

            Ok(t_consq)
        }

        Expr::IsZero(e) => {
            let t = let_type_of_expr(e, tenv)?;
            if t.is_int() {
                Ok(LetType::new_bool())
            } else {
                let msg = format!("`zero?` expects `int` but got `{t}`");
                Err(msg)
            }
        }

        Expr::Let(var, initializer, body) => {
            let t_var = let_type_of_expr(initializer, tenv)?;
            tenv.push(var, &t_var);
            let t_body = let_type_of_expr(body, tenv)?;
            tenv.pop();
            Ok(t_body)
        }

        Expr::LiteralBool(_) => Ok(LetType::new_bool()),

        Expr::Proc(var, t_var, body) => {
            tenv.push(var, t_var);
            let t_body = let_type_of_expr(body, tenv)?;
            tenv.pop();
            let t_proc = LetType::new_proc(t_var.clone(), t_body);
            Ok(t_proc)
        }

        Expr::LetRec {
            result_type,
            name,
            var,
            var_type,
            proc_body,
            let_body,
        } => {
            let t_proc = LetType::new_proc(var_type.clone(), result_type.clone());
            tenv.push(name, &t_proc);
            tenv.push(var, var_type);
            let t_body = let_type_of_expr(proc_body, tenv)?;
            if t_body != *result_type {
                let msg =
                    format!("`{name}` expect result of type `{result_type}` but got `{t_body}`.");
                return Err(msg);
            }
            tenv.pop();
            let t_let_body = let_type_of_expr(let_body, tenv)?;
            tenv.pop();
            Ok(t_let_body)
        }

        Expr::Var(var) => {
            if let Some(t_var) = tenv.lookup(var) {
                Ok(t_var.clone())
            } else {
                let msg = format!("undefined name `{var}`");
                Err(msg)
            }
        }
    }
}

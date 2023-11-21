//! Type checks a letpl program

use crate::ast::{Expr, Program, Type};
use crate::table::Table;

pub fn let_type_of(program: &Program) -> Result<Type, String> {
    let mut tenv = Table::new();
    let_type_of_expr(&program.expr, &mut tenv)
}

fn let_type_of_expr(expr: &Expr, tenv: &mut Table<Type>) -> Result<Type, String> {
    match expr {
        Expr::Assert { test, body, .. } => {
            let t_guard = let_type_of_expr(test, tenv)?;
            if !t_guard.is_bool() {
                let msg = format!("assert guard must be type `bool` but got `{t_guard}`");
                return Err(msg);
            }
            let_type_of_expr(body, tenv)
        }

        Expr::Call { proc, arg } => {
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

        Expr::LiteralInt(_) => Ok(Type::new_int()),

        Expr::Subtract { left, right } => {
            let t1 = let_type_of_expr(left, tenv)?;
            if !t1.is_int() {
                let msg = format!("-() first argument expects `int` but got `{t1}`");
                return Err(msg);
            }
            let t2 = let_type_of_expr(right, tenv)?;
            if !t2.is_int() {
                let msg = format!("-() second argument expects `int` but got `{t2}`");
                return Err(msg);
            }
            Ok(Type::new_int())
        }

        Expr::If {
            test,
            consequent,
            alternate,
        } => {
            let t_test = let_type_of_expr(test, tenv)?;
            if !t_test.is_bool() {
                let msg = format!("`if` test expects `bool` but got `{t_test}`");
                return Err(msg);
            }

            let t_consequent = let_type_of_expr(consequent, tenv)?;
            let t_alternate = let_type_of_expr(alternate, tenv)?;
            if t_consequent != t_alternate {
                let msg = format!(
                    "`if` branches expect matching types but got `{t_consequent}` and `{t_alternate}`"
                );
                return Err(msg);
            }

            Ok(t_consequent)
        }

        Expr::IsZero(e) => {
            let t = let_type_of_expr(e, tenv)?;
            if t.is_int() {
                Ok(Type::new_bool())
            } else {
                let msg = format!("`zero?` expects `int` but got `{t}`");
                Err(msg)
            }
        }

        Expr::Let { name, expr, body } => {
            let t_expr = let_type_of_expr(expr, tenv)?;
            tenv.push(name.clone(), t_expr);
            let t_body = let_type_of_expr(body, tenv)?;
            tenv.pop();
            Ok(t_body)
        }

        Expr::LiteralBool(_) => Ok(Type::new_bool()),

        Expr::Proc { param, body } => {
            tenv.push(param.name.clone(), param.t.clone());
            let t_body = let_type_of_expr(body, tenv)?;
            tenv.pop();
            let t_proc = Type::new_proc(param.t.clone(), t_body);
            Ok(t_proc)
        }

        Expr::LetRec {
            t_result,
            name,
            param,
            proc_body,
            let_body,
        } => {
            let t_proc = Type::new_proc(param.t.clone(), t_result.clone());
            tenv.push(name.clone(), t_proc);
            tenv.push(param.name.clone(), param.t.clone());
            let t_body = let_type_of_expr(proc_body, tenv)?;
            if t_body != *t_result {
                let msg =
                    format!("`{name}` expect result of type `{t_result}` but got `{t_body}`.");
                return Err(msg);
            }
            tenv.pop();
            let t_let_body = let_type_of_expr(let_body, tenv)?;
            tenv.pop();
            Ok(t_let_body)
        }

        Expr::Name(name) => {
            if let Some(t_name) = tenv.lookup(name) {
                Ok(t_name.clone())
            } else {
                let msg = format!("undefined name `{name}`");
                Err(msg)
            }
        }
    }
}

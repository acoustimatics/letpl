//! Type checks a letpl program

use crate::ast::{Expr, Program, Type};
use crate::table::Table;

pub fn type_of_program(program: &Program) -> Result<Type, String> {
    let mut env = Table::new();
    type_of_expr(&program.expr, &mut env)
}

fn type_of_expr(expr: &Expr, env: &mut Table<Type>) -> Result<Type, String> {
    match expr {
        Expr::Assert { test, body, .. } => {
            let t_test = type_of_expr(test, env)?;
            if !t_test.is_bool() {
                let msg = format!("assert guard must be type `bool` but got `{t_test}`");
                return Err(msg);
            }
            type_of_expr(body, env)
        }

        Expr::Call { proc, arg } => {
            let t_proc = type_of_expr(proc, env)?;
            let Some((t_param, t_body)) = t_proc.as_proc() else {
                let msg = format!("call expects proc but got `{t_proc}`");
                return Err(msg);
            };
            let t_arg = type_of_expr(arg, env)?;
            if t_param != &t_arg {
                let msg = format!("call expect `{t_param}` argument but got `{t_arg}`");
                return Err(msg);
            }
            Ok(t_body.clone())
        }

        Expr::LiteralInt(_) => Ok(Type::new_int()),

        Expr::Subtract { left, right } => {
            let t_left = type_of_expr(left, env)?;
            if !t_left.is_int() {
                let msg = format!("-() first argument expects `int` but got `{t_left}`");
                return Err(msg);
            }
            let t_right = type_of_expr(right, env)?;
            if !t_right.is_int() {
                let msg = format!("-() second argument expects `int` but got `{t_right}`");
                return Err(msg);
            }
            Ok(Type::new_int())
        }

        Expr::If {
            test,
            consequent,
            alternate,
        } => {
            let t_test = type_of_expr(test, env)?;
            if !t_test.is_bool() {
                let msg = format!("`if` test expects `bool` but got `{t_test}`");
                return Err(msg);
            }

            let t_consequent = type_of_expr(consequent, env)?;
            let t_alternate = type_of_expr(alternate, env)?;
            if t_consequent != t_alternate {
                let msg = format!(
                    "`if` branches expect matching types but got `{t_consequent}` and `{t_alternate}`"
                );
                return Err(msg);
            }

            Ok(t_consequent)
        }

        Expr::IsZero(expr) => {
            let t_expr = type_of_expr(expr, env)?;
            if t_expr.is_int() {
                Ok(Type::new_bool())
            } else {
                let msg = format!("`zero?` expects `int` but got `{t_expr}`");
                Err(msg)
            }
        }

        Expr::Let { name, expr, body } => {
            let t_expr = type_of_expr(expr, env)?;
            env.push(name.clone(), t_expr);
            let t_body = type_of_expr(body, env)?;
            env.pop();
            Ok(t_body)
        }

        Expr::LiteralBool(_) => Ok(Type::new_bool()),

        Expr::Proc { param, body } => {
            env.push(param.name.clone(), param.t.clone());
            let t_body = type_of_expr(body, env)?;
            env.pop();
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
            env.push(name.clone(), t_proc);
            env.push(param.name.clone(), param.t.clone());
            let t_body = type_of_expr(proc_body, env)?;
            if t_body != *t_result {
                let msg =
                    format!("`{name}` expect result of type `{t_result}` but got `{t_body}`.");
                return Err(msg);
            }
            env.pop();
            let t_let_body = type_of_expr(let_body, env)?;
            env.pop();
            Ok(t_let_body)
        }

        Expr::Name(name) => {
            if let Some(t_name) = env.lookup(name) {
                Ok(t_name.clone())
            } else {
                let msg = format!("undefined name `{name}`");
                Err(msg)
            }
        }
    }
}

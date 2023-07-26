use crate::parser::{LetType, Program, Expr};

pub fn let_type_of(program: &Program) -> Result<LetType, String> {
    let_type_of_expr(&program.expr)
}

fn let_type_of_expr(expr: &Expr) -> Result<LetType, String> {
    match expr {
        Expr::Const(_) => {
            Ok(LetType::new_int())
        }

        Expr::Diff(e1, e2) => {
            let t1 = let_type_of_expr(e1)?;
            if !t1.is_int() {
                let msg = format!("-() first argument expects `int` but got `{}`", t1);
                return Err(msg)
            }
            let t2 = let_type_of_expr(e2)?;
            if !t2.is_int() {
                let msg = format!("-() second argument expects `int` but got `{}`", t2);
                return Err(msg)
            }
            Ok(LetType::new_int())
        }

        Expr::If(cond, consq, alt) => {
            let t_cond = let_type_of_expr(cond)?;
            if !t_cond.is_bool() {
                let msg = format!("`if` condition expects `bool` but got `{}`", t_cond);
                return Err(msg);
            }

            let t_consq = let_type_of_expr(consq)?;
            let t_alt = let_type_of_expr(alt)?;
            if t_consq != t_alt {
                let msg = format!("`if` branches expect matching types but got `{}` and `{}`", t_consq, t_alt);
                return Err(msg);
            }

            Ok(t_consq)
        }

        Expr::IsZero(e) => {
            let t = let_type_of_expr(e)?;
            if t.is_int() {
                Ok(LetType::new_bool())
            } else {
                let msg = format!("`zero?` expects `int` but got `{}`", t);
                Err(msg)
            }
        }

        Expr::Print(e) => {
            let_type_of_expr(e)
        }

        Expr::Proc(_, t_var, body) => {
            let t_body = let_type_of_expr(body)?;
            let t_proc = LetType::new_proc(t_var.clone(), t_body);
            Ok(t_proc)
        }

        _ => unimplemented!(),
    }
}

use crate::environment::Environment;
use crate::parser;
use crate::syntax::{Expr, Program};
use crate::value::Value;

pub type RunResult = Result<Value, String>;

/// Parses and interprets a given source text returning it resulting value.
pub fn run(src: &str) -> RunResult {
    let program = parser::parse(src)?;
    let value = value_of_program(&program)?;
    Ok(value)
}

fn value_of_program(program: &Program) -> RunResult {
    value_of(&program.expr, &Environment::empty())
}

fn value_of(expr: &Expr, env: &Environment<Value>) -> RunResult {
    match expr {
        Expr::Const(x) => Ok(Value::Number(*x)),
        Expr::Diff(expr1, expr2) => {
            let val1 = value_of(&expr1, env)?;
            let val2 = value_of(&expr2, env)?;
            diff(val1, val2)
        }
        Expr::IsZero(expr) => {
            let val = value_of(expr, env)?;
            is_zero(val)
        }
        Expr::If(guard, consq, alt) => {
            let guard_val = value_of(guard, env)?;
            if is_true(guard_val)? {
                value_of(consq, env)
            } else {
                value_of(alt, env)
            }
        }
        Expr::Var(var) => {
            if let Some(val) = env.apply(var) {
                Ok(*val)
            } else {
                let msg = format!("unbound variable `{}`", var);
                Err(msg)
            }
        }
        Expr::Let(var, expr, body) => {
            let val = value_of(expr, env)?;
            let env = env.extend(var.clone(), val);
            value_of(body, &env)
        }
    }
}

fn diff(v1: Value, v2: Value) -> RunResult {
    match (v1, v2) {
        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
        _ => Err(String::from("-() requires numerical values")),
    }
}

fn is_zero(v: Value) -> RunResult {
    if let Value::Number(n) = v {
        return Ok(Value::Boolean(n == 0.0));
    } else {
        return Err(String::from("zero?() requires a numeric value"));
    }
}

fn is_true(v: Value) -> Result<bool, String> {
    if let Value::Boolean(b) = v {
        Ok(b)
    } else {
        Err(String::from("if requires Boolean guard"))
    }
}

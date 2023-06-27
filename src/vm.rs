use crate::chunk::Chunk;
use crate::environment::Environment;
use crate::op::*;
use crate::value::Value;

/// Runs a chunk of code which represents an expression returning its value.
pub fn run(chunk: &Chunk) -> Result<Value, String> {
    let mut ip = 0;
    let mut stack: Vec<Value> = Vec::new();
    let mut env: Environment<Value> = Environment::empty();

    while ip < chunk.ops.len() {
        let op = &chunk.ops[ip];
        ip += 1;

        match op {
            Op::Branch(i) => {
                ip = *i;
            }
            Op::BranchTrue(i) => {
                if stack.pop().unwrap().as_bool()? {
                    ip = *i;
                }
            }
            Op::Apply(name) => {
                match env.apply(name) {
                    Some(v) => stack.push(*v),
                    None => {
                        let msg = format!("unbound identifier `{}`", name);
                        return Err(msg);
                    }
                }
            }
            Op::Bind(name) => {
                let v = stack.pop().unwrap();
                env = env.extend(name.clone(), v);
            }
            Op::Diff => {
                let x2 = stack.pop().unwrap().as_number()?;
                let x1 = stack.pop().unwrap().as_number()?;
                let v = Value::Number(x1 - x2);
                stack.push(v);
            }
            Op::IsZero => {
                let x = stack.pop().unwrap().as_number()?;
                let v = Value::Boolean(x == 0.0);
                stack.push(v);
            }
            Op::PushValue(v) => {
                stack.push(*v);
            }
            Op::Unbind => {
                env = env.retract().unwrap();
            }
        }
    }

    Ok(stack[0])
}

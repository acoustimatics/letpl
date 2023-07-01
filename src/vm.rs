use crate::chunk::Chunk;
use crate::environment::Environment;
use crate::op::*;
use crate::procedure::Procedure;
use crate::value::Value;

struct Frame {
    ip: usize,
    env: Environment<Value>,
}

impl Frame {
    fn new(ip: usize, env: Environment<Value>) -> Frame {
        Frame { ip, env }
    }
}

macro_rules! pop {
    ($stack:expr) => {
        $stack.pop().unwrap()
    };
}

/// Runs a chunk of code which represents an expression returning its value.
pub fn run(chunk: &Chunk) -> Result<Value, String> {
    let mut stack: Vec<Value> = Vec::new();
    let mut call_stack: Vec<Frame> = Vec::new();
    let mut ip = 0;
    let mut env: Environment<Value> = Environment::empty();

    while ip < chunk.ops.len() {
        let op = &chunk.ops[ip];
        ip += 1;

        match op {
            Op::Jump(i) => {
                ip = *i;
            }
            Op::JumpTrue(i) => {
                if pop!(stack).as_bool()? {
                    ip = *i;
                }
            }
            Op::Apply(name) => match env.apply(name) {
                Some(v) => stack.push(v.clone()),
                None => {
                    let msg = format!("unbound identifier `{}`", name);
                    return Err(msg);
                }
            },
            Op::Bind(name) => {
                let v = pop!(stack);
                env = env.extend(name, v);
            }
            Op::Diff => {
                let x2 = pop!(stack).as_number()?;
                let x1 = pop!(stack).as_number()?;
                let v = Value::Number(x1 - x2);
                stack.push(v);
            }
            Op::IsZero => {
                let x = pop!(stack).as_number()?;
                let v = Value::Boolean(x == 0.0);
                stack.push(v);
            }
            Op::PushValue(v) => {
                stack.push(v.clone());
            }
            Op::Unbind => {
                env = env.retract().unwrap();
            }
            Op::Return => {
                let frame = pop!(call_stack);
                ip = frame.ip;
                env = frame.env;
            }
            Op::Call => {
                call_stack.push(Frame::new(ip, env));

                let arg = pop!(stack);
                let proc_value = pop!(stack);

                let proc = proc_value.as_proc()?;
                ip = proc.start;
                env = proc.env.extend(&proc.var, arg);
            }
            Op::MakeProc(var, start) => {
                let proc = Procedure::new(var, *start, &env);
                let value = Value::Procedure(proc);
                stack.push(value);
            }
        }
    }

    Ok(pop!(stack))
}

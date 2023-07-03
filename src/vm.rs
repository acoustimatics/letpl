use std::rc::Rc;

use crate::chunk::Chunk;
use crate::environment::Environment;
use crate::op::*;
use crate::procedure::Procedure;
use crate::value::Value;

struct Frame {
    i_op: usize,
    env: Environment,
}

impl Frame {
    fn new(i_op: usize, env: Environment) -> Frame {
        Frame { i_op, env }
    }
}

macro_rules! pop {
    ($stack:expr) => {
        $stack.pop().unwrap()
    };
}

macro_rules! pop_number {
    ($stack:expr) => {
        $stack.pop().unwrap().as_number()
    };
}

/// Runs a chunk of code which represents an expression returning its value.
pub fn run(chunk: &Chunk) -> Result<Value, String> {
    let mut stack: Vec<Value> = Vec::new();
    let mut call_stack: Vec<Frame> = Vec::new();
    let mut i_op = 0;
    let mut env = Environment::empty();

    while i_op < chunk.ops.len() {
        let op = &chunk.ops[i_op];
        i_op += 1;

        match op {
            Op::Jump(i) => {
                i_op = *i;
            }
            Op::JumpTrue(i) => {
                if pop!(stack).as_bool()? {
                    i_op = *i;
                }
            }
            Op::Apply(name) => match env.fetch(name) {
                Some(v) => stack.push(v.clone()),
                None => {
                    let msg = format!("unbound identifier `{}`", name);
                    return Err(msg);
                }
            },
            Op::Bind(name) => {
                let v = pop!(stack);
                env.push(name, v);
            }
            Op::Diff => {
                let x2 = pop_number!(stack)?;
                let x1 = pop_number!(stack)?;
                let v = Value::Number(x1 - x2);
                stack.push(v);
            }
            Op::IsZero => {
                let x = pop_number!(stack)?;
                let v = Value::Boolean(x == 0.0);
                stack.push(v);
            }
            Op::Pop => {
                let _ = pop!(stack);
            }
            Op::PushValue(v) => {
                stack.push(v.clone());
            }
            Op::Unbind => {
                env.pop()?;
            }
            Op::Return => {
                let frame = pop!(call_stack);
                i_op = frame.i_op;
                env = frame.env;
            }
            Op::Call => {
                let calling_frame = Frame::new(i_op, env);
                call_stack.push(calling_frame);

                let p = stack[stack.len() - 2].as_proc()?;
                i_op = p.start;
                env = p.env.clone();
            }
            Op::MakeProc(start) => {
                let env = env.clone();
                let p = Procedure::new(*start, env);
                let p = Rc::new(p);
                let v = Value::Procedure(p);
                stack.push(v);
            }
            Op::Minus => {
                let x = pop_number!(stack)?;
                let v = Value::Number(-x);
                stack.push(v);
            }
        }
    }

    Ok(pop!(stack))
}

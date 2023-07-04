use std::rc::Rc;

use crate::chunk::{Address, Chunk};
use crate::op::Op;
use crate::procedure::Procedure;
use crate::value::Value;

struct Frame {
    next_op: Address,
    env: Vec<Value>,
}

impl Frame {
    fn new(next_op: Address, env: Vec<Value>) -> Frame {
        Frame { next_op, env }
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

pub fn run(chunk: &Chunk) -> Result<Value, String> {
    let mut stack: Vec<Value> = Vec::new();
    let mut call_stack: Vec<Frame> = Vec::new();
    let mut next_op: Address = 0;
    let mut env: Vec<Value> = Vec::new();

    while next_op < chunk.ops.len() {
        let op = &chunk.ops[next_op];
        next_op += 1;

        match op {
            Op::Bind => {
                let v = pop!(stack);
                env.push(v);
            }

            Op::Call => {
                let calling_frame = Frame::new(next_op, env);
                call_stack.push(calling_frame);

                let p = stack[stack.len() - 2].as_proc()?;
                next_op = p.start;
                env = p.env.clone();
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

            Op::Jump(address) => {
                next_op = *address;
            }

            Op::JumpTrue(address) => {
                if pop!(stack).as_bool()? {
                    next_op = *address;
                }
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

            Op::Pop => {
                let _ = pop!(stack);
            }

            Op::PushBinding(scope) => {
                let v = env[*scope].clone();
                stack.push(v);
            }

            Op::PushValue(value) => {
                stack.push(value.clone());
            }

            Op::Return => {
                let frame = pop!(call_stack);
                next_op = frame.next_op;
                env = frame.env;
            }

            Op::Unbind => {
                let _ = pop!(env);
            }
        }
    }

    Ok(pop!(stack))
}

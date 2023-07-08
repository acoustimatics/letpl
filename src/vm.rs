use std::rc::Rc;

use crate::chunk::{Address, Chunk};
use crate::op::{Capture, Op};
use crate::procedure::Procedure;
use crate::value::Value;

struct Frame {
    next_op: Address,
    env: Vec<Value>,
    captures: Rc<Vec<Value>>,
}

impl Frame {
    fn new(next_op: Address, env: Vec<Value>, captures: Rc<Vec<Value>>) -> Self {
        Self { next_op, env, captures }
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
    let mut stack = Vec::<Value>::new();
    let mut call_stack = Vec::<Frame>::new();

    let mut next_op: Address = 0;
    let mut env = Vec::<Value>::new();
    let mut captures = Rc::new(Vec::<Value>::new());
 
    while next_op < chunk.ops.len() {
        let op = &chunk.ops[next_op];
        next_op += 1;

        match op {
            Op::Bind => {
                let v = pop!(stack);
                env.push(v);
            }

            Op::Call => {
                let calling_frame = Frame::new(next_op, env, captures);
                call_stack.push(calling_frame);

                let p = stack[stack.len() - 2].as_proc()?;
                next_op = p.start;
                env = Vec::new();
                captures = Rc::clone(&p.captures);
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

            Op::MakeProc(start, capture_ops) => {
                let proc_captures: Vec<Value> = capture_ops
                    .iter()
                    .map(|c| {
                        match c {
                            Capture::Local(scope) => env[*scope].clone(),
                            Capture::Capture(index) => captures[*index].clone(),
                        }
                    })
                    .collect();
                let proc = Procedure::new(*start, proc_captures);
                let proc = Rc::new(proc);
                let value = Value::Procedure(proc);
                stack.push(value);
            }

            Op::Minus => {
                let x = pop_number!(stack)?;
                let v = Value::Number(-x);
                stack.push(v);
            }

            Op::Pop => {
                let _ = pop!(stack);
            }

            Op::PushCapture(index) => {
                let v = captures[*index].clone();
                stack.push(v);
            }

            Op::PushLocal(scope) => {
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
                captures = frame.captures;
            }
        }
    }

    Ok(pop!(stack))
}

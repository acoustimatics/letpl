use std::rc::Rc;

use crate::chunk::{Address, Chunk};
use crate::op::{Capture, Op};
use crate::procedure::Procedure;
use crate::value::Value;

struct Frame {
    next_op: Address,
    stack_index: usize,
    captures: Rc<Vec<Value>>,
}

impl Frame {
    fn new(next_op: Address, stack_index: usize, captures: Rc<Vec<Value>>) -> Self {
        Self {
            next_op,
            stack_index,
            captures,
        }
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
    dbg!(chunk);
    let mut stack = Vec::<Value>::new();
    let mut call_stack = Vec::<Frame>::new();

    let mut next_op: Address = 0;
    let mut frame_stack_index = 0;
    let mut captures = Rc::new(Vec::<Value>::new());

    while next_op < chunk.ops.len() {
        let op = &chunk.ops[next_op];
        next_op += 1;

        match op {
            Op::Call => {
                let calling_frame = Frame::new(next_op, frame_stack_index, captures);
                call_stack.push(calling_frame);

                frame_stack_index = stack.len() - 2;
                let p = stack[frame_stack_index].as_proc()?;
                next_op = p.start;
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
                    .map(|c| match c {
                        Capture::Local(index) => {
                            stack[frame_stack_index + index].clone()
                        }
                        Capture::Capture(index) => captures[*index].clone(),
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

            Op::PushCapture(capture_index) => {
                let v = captures[*capture_index].clone();
                stack.push(v);
            }

            Op::PushLocal(stack_index) => {
                let v = stack[frame_stack_index + stack_index].clone();
                stack.push(v);
            }

            Op::PushValue(value) => {
                stack.push(value.clone());
            }

            Op::Return => {
                let return_value = stack[stack.len() - 1].clone();
                for _ in frame_stack_index..stack.len() {
                    pop!(stack);
                }
                stack.push(return_value);

                let frame = pop!(call_stack);
                next_op = frame.next_op;
                frame_stack_index = frame.stack_index;
                captures = frame.captures;
            }
        }
    }

    dbg!(stack.len());

    Ok(pop!(stack))
}

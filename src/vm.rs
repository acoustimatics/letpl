use std::rc::Rc;

use crate::chunk::Chunk;
use crate::op::Op;
use crate::procedure::Procedure;
use crate::value::Value;

struct Frame {
    i_op: usize,
    env: Vec<Value>,
}

impl Frame {
    fn new(i_op: usize, env: Vec<Value>) -> Frame {
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
    let mut env: Vec<Value> = Vec::new();

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
            Op::PushBinding(i) => {
                let v = env[*i].clone();
                stack.push(v);
            }
            Op::Bind => {
                let v = pop!(stack);
                env.push(v);
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
                let _ = pop!(env);
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

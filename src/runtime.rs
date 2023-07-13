use std::fmt;
use std::rc::Rc;

/// Represents a procedure and its captured environment.
pub struct Procedure {
    start: usize,
    captures: Rc<Vec<Value>>,
}

impl Procedure {
    fn new(start: usize, captures: Vec<Value>) -> Self {
        let captures = Rc::new(captures);
        Self { start, captures }
    }
}

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<proc @{}>", self.start)
    }
}

/// Represents final value to which an expression can evalutate.
#[derive(Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Procedure(Rc<Procedure>),
}

impl Value {
    pub fn as_number(&self) -> Result<f64, String> {
        match self {
            Value::Number(x) => Ok(*x),
            _ => Err(String::from("value is not a number")),
        }
    }

    pub fn as_bool(&self) -> Result<bool, String> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => Err(String::from("value is not a boolean")),
        }
    }

    pub fn as_proc(&self) -> Result<&Procedure, String> {
        match self {
            Value::Procedure(p) => Ok(p),
            _ => Err(String::from("value is not a procedure")),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(x) => write!(f, "{}", x),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Procedure(p) => write!(f, "{}", p),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub enum Capture {
    Local(usize),
    Capture(usize),
}

/// Represents a VM operation.
#[derive(Debug)]
pub enum Op {
    /// Call a procedure. Call expects two values at the on the stack: at the
    /// top an argument and next a procedure. Save he current op index and
    /// environment to the call stack, then set the instruction index and
    /// environment to the procedure's start and environment, respectively. The
    /// procedure's code must pop the argument and procedure from the stack.
    Call,

    /// Pop two numbers from the stack, subtract them, and push the difference
    /// onto the stack.
    Diff,

    /// Pop a number from the stack. If the popped value is zero then push
    /// `true` onto the stack, otherwise push `false`.
    IsZero,

    /// Unconditionally jump to an index.
    Jump(usize),

    /// Pop a Boolean from the stack. If the popped value is `true` then jump to
    /// an index.
    JumpTrue(usize),

    /// Make a procedure using a start index and the environment. Push the
    /// procedure onto the stack.
    MakeProc(usize, Vec<Capture>),

    /// Pop a number from the stack and push its negative onto the stack.
    Minus,

    PushCapture(usize),

    /// Pushes a environment binding onto the stack.
    PushLocal(usize),

    /// Push a value onto the stack.
    PushValue(Value),

    /// Return from a procedure. Pop the op index and environment from the call
    /// stack. For the return value, the procedure's code must have left one
    /// value on the stack.
    Return,
}

struct Frame {
    next_op: usize,
    stack_index: usize,
    captures: Rc<Vec<Value>>,
}

impl Frame {
    fn new(next_op: usize, stack_index: usize, captures: Rc<Vec<Value>>) -> Self {
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

pub fn run(program: &[Op]) -> Result<Value, String> {
    let mut stack = Vec::<Value>::new();
    let mut call_stack = Vec::<Frame>::new();

    let mut next_op = 0;
    let mut frame_stack_index = 0;
    let mut captures = Rc::new(Vec::<Value>::new());

    while next_op < program.len() {
        let op = &program[next_op];
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
                        Capture::Local(index) => stack[frame_stack_index + index].clone(),
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

    Ok(pop!(stack))
}
//! A stack-based bytecode VM.

use std::fmt;
use std::rc::Rc;

use crate::offset::{CaptureOffset, StackOffset};

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
    Integer(i64),
    Boolean(bool),
    Procedure(Rc<Procedure>),
}

impl Value {
    pub fn as_int(&self) -> Result<i64, String> {
        match self {
            Value::Integer(x) => Ok(*x),
            _ => Err(String::from("value is not an integer")),
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
            Value::Integer(x) => write!(f, "{x}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Procedure(p) => write!(f, "{p}"),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
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
    /// Pop a Boolean from the stack. If the value is false then halt execution
    /// and include the line number in the error message.
    Assert {
        line: usize,
    },

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

    /// Pushes a captured value onto the stack.
    PushCapture(CaptureOffset),

    PushGlobal(usize),

    /// Pushes a environment binding onto the stack.
    PushLocal(StackOffset),

    /// Push a value onto the stack.
    PushValue(Value),

    /// Return from a procedure. Pop the op index and environment from the call
    /// stack. For the return value, the procedure's code must have left one
    /// value on the stack.
    Return,

    TailCall,
}

struct Frame {
    next_op: usize,
    stack_base: StackOffset,
    captures: Rc<Vec<Value>>,
}

impl Frame {
    fn new(next_op: usize, stack_base: StackOffset, captures: Rc<Vec<Value>>) -> Self {
        Self {
            next_op,
            stack_base,
            captures,
        }
    }
}

struct ValueStack {
    stack: Vec<Value>,
}

impl ValueStack {
    fn new() -> Self {
        let stack = Vec::new();
        Self { stack }
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Result<Value, String> {
        if let Some(value) = self.stack.pop() {
            Ok(value)
        } else {
            Err(String::from("stack underflow"))
        }
    }

    fn pop_bool(&mut self) -> Result<bool, String> {
        self.pop()?.as_bool()
    }

    fn pop_int(&mut self) -> Result<i64, String> {
        self.pop()?.as_int()
    }

    fn pop_to(&mut self, base: StackOffset) -> Result<(), String> {
        let StackOffset(base) = base;
        let top = self.stack.len();
        for _ in base..top {
            if let None = self.stack.pop() {
                return Err(String::from("stack underflow"));
            }
        }

        Ok(())
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    fn value_at(&self, base: StackOffset, offset: StackOffset) -> &Value {
        let StackOffset(absolute_offset) = base + offset;
        &self.stack[absolute_offset]
    }
}

pub fn run(program: &[Op]) -> Result<Value, String> {
    let mut stack = ValueStack::new();
    let mut call_stack = Vec::<Frame>::new();

    let mut next_op = 0;
    let mut stack_base = StackOffset(0);
    let mut captures = Rc::new(Vec::<Value>::new());

    while next_op < program.len() {
        let op = &program[next_op];
        next_op += 1;

        match op {
            Op::Assert { line } => {
                if !stack.pop_bool()? {
                    let msg = format!("Assert at line {line}");
                    return Err(msg.to_string());
                }
            }

            Op::Call => {
                let calling_frame = Frame::new(next_op, stack_base, captures);
                call_stack.push(calling_frame);

                stack_base = StackOffset(stack.len() - 2);

                let p = stack.value_at(stack_base, StackOffset(0)).as_proc()?;

                next_op = p.start;
                captures = Rc::clone(&p.captures);
            }

            Op::Diff => {
                let x2 = stack.pop_int()?;
                let x1 = stack.pop_int()?;
                let v = Value::Integer(x1 - x2);
                stack.push(v);
            }

            Op::IsZero => {
                let x = stack.pop_int()?;
                let v = Value::Boolean(x == 0);
                stack.push(v);
            }

            Op::Jump(address) => {
                next_op = *address;
            }

            Op::JumpTrue(address) => {
                if stack.pop_bool()? {
                    next_op = *address;
                }
            }

            Op::MakeProc(start, capture_ops) => {
                let proc_captures: Vec<Value> = capture_ops
                    .iter()
                    .map(|c| match c {
                        Capture::Local(index) => {
                            stack.value_at(stack_base, StackOffset(*index)).clone()
                        }
                        Capture::Capture(index) => captures[*index].clone(),
                    })
                    .collect();
                let proc = Procedure::new(*start, proc_captures);
                let proc = Rc::new(proc);
                let value = Value::Procedure(proc);
                stack.push(value);
            }

            Op::PushCapture(CaptureOffset(capture_offset)) => {
                let v = captures[*capture_offset].clone();
                stack.push(v);
            }

            Op::PushGlobal(stack_index) => {
                let v = stack
                    .value_at(StackOffset(*stack_index), StackOffset(0))
                    .clone();
                stack.push(v);
            }

            Op::PushLocal(offset) => {
                let v = stack.value_at(stack_base, *offset).clone();
                stack.push(v);
            }

            Op::PushValue(value) => {
                stack.push(value.clone());
            }

            Op::Return => {
                let return_value = stack
                    .value_at(StackOffset(stack.len() - 1), StackOffset(0))
                    .clone();
                stack.pop_to(stack_base)?;
                stack.push(return_value);

                let Some(frame) = call_stack.pop() else {
                    return Err(String::from("call stack underflow"));
                };
                next_op = frame.next_op;
                stack_base = frame.stack_base;
                captures = frame.captures;
            }

            Op::TailCall => {
                let argument = stack.pop()?;
                let proc = stack.pop()?;

                // Cleanup stack frame.
                stack.pop_to(stack_base)?;

                // Set up a jump to procedure.
                {
                    let p = proc.as_proc()?;
                    next_op = p.start;
                    captures = Rc::clone(&p.captures);
                }

                // Setup stack so it looks like the proc was called instead of
                // jumped to.
                stack.push(proc);
                stack.push(argument);
            }
        }
    }

    stack.pop()
}

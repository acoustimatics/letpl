use crate::value::Value;
use std::fmt;

/// Represents a VM operation.
pub enum Op {
    /// Push a value bound to a name in the environment onto the stack.
    Apply(String),

    /// Pop a value from the stack and bind the value to a name in the
    /// environment.
    Bind(String),

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
    MakeProc(usize),

    /// Pop a number from the stack and push its negative onto the stack.
    Minus,

    /// Pop a value from the stack and discard it.
    Pop,

    /// Push a value onto the stack.
    PushValue(Value),

    /// Return from a procedure. Pop the op index and environment from the call
    /// stack. For the return value, the procedure's code must have left one
    /// value on the stack.
    Return,

    /// Remove the last binding in the environment.
    Unbind,
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Apply(name) => write!(f, "apply {}", name),
            Op::Bind(name) => write!(f, "bind {}", name),
            Op::Call => write!(f, "call"),
            Op::Diff => write!(f, "diff"),
            Op::IsZero => write!(f, "is-zero"),
            Op::Jump(i) => write!(f, "jump {}", i),
            Op::JumpTrue(i) => write!(f, "jump-true {}", i),
            Op::MakeProc(i) => write!(f, "make-proc @{}", i),
            Op::Minus => write!(f, "minus"),
            Op::Pop => write!(f, "pop"),
            Op::PushValue(v) => write!(f, "push-value {}", v),
            Op::Return => write!(f, "return"),
            Op::Unbind => write!(f, "unbind"),
        }
    }
}

use crate::binding::Scope;
use crate::chunk::Address;
use crate::value::Value;

/// Represents a VM operation.
#[derive(Debug)]
pub enum Op {
    /// Pops the stack and creates a new environment binding.
    Bind,

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
    Jump(Address),

    /// Pop a Boolean from the stack. If the popped value is `true` then jump to
    /// an index.
    JumpTrue(Address),

    /// Make a procedure using a start index and the environment. Push the
    /// procedure onto the stack.
    MakeProc(Address),

    /// Pop a number from the stack and push its negative onto the stack.
    Minus,

    /// Pop a value from the stack and discard it.
    Pop,

    /// Pushes a environment binding onto the stack.
    PushBinding(Scope),

    /// Push a value onto the stack.
    PushValue(Value),

    /// Return from a procedure. Pop the op index and environment from the call
    /// stack. For the return value, the procedure's code must have left one
    /// value on the stack.
    Return,

    /// Remove the last binding in the environment.
    Unbind,
}

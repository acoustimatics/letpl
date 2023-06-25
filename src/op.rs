use crate::value::Value;
use std::fmt;

/// Represents a VM operation.
pub enum Op {
    /// Pushes a value bound to a name in the environment onto the stack.
    Apply(String),

    /// Pops a value from the stack and binds the value to a name in the
    /// environment.
    Bind(String),

    /// Unconditionally branches to an index.
    Branch(usize),

    /// Pops the stack. If the popped value is `true` then branch to an index.
    BranchTrue(usize),

    /// Pop two values from the stack, subtract them, and push the
    /// difference on the stack.
    Diff,

    /// Pop the stack. If the popped value is zero then push `true` onto the
    /// stack, otherwise push `false`.
    IsZero,

    /// Pushes an immediate value onto the stack.
    PushValue(Value),

    /// Removes the last binding in the environment.
    Unbind,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Apply(name) => write!(f, "apply {}", name),
            Op::Bind(name) => write!(f, "bind {}", name),
            Op::Branch(i) => write!(f, "branch {}", i),
            Op::BranchTrue(i) => write!(f, "branch-true {}", i),
            Op::Diff => write!(f, "diff"),
            Op::IsZero => write!(f, "is-zero"),
            Op::PushValue(v) => write!(f, "push-value {}", v),
            Op::Unbind => write!(f, "unbind"),
        }
    }
}

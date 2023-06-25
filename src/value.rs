use std::fmt;

/// Represents final value to which an expression can evalutate.
#[derive(Clone, Copy)]
pub enum Value {
    Number(i64),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

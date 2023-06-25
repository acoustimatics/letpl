use std::fmt;

/// Represents final value to which an expression can evalutate.
#[derive(Clone, Copy)]
pub enum Value {
    Number(f64),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(x) => write!(f, "{}", x),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

use std::fmt;

/// Represents final value to which an expression can evalutate.
#[derive(Clone, Copy)]
pub enum Value {
    Number(f64),
    Boolean(bool),
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
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(x) => write!(f, "{}", x),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

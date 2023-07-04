use std::fmt;

use crate::chunk::Address;
use crate::value::Value;

/// Represents a procedure and its captured environment.
pub struct Procedure {
    pub start: Address,
    pub env: Vec<Value>,
}

impl Procedure {
    pub fn new(start: Address, env: Vec<Value>) -> Self {
        Procedure { start, env }
    }
}

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<proc @{}>", self.start)
    }
}

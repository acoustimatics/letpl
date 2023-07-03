use std::fmt;

use crate::value::Value;

/// Represents a procedure and its captured environment.
pub struct Procedure {
    /// The starting index.
    pub start: usize,

    /// The environment when the procedure object was made.
    pub env: Vec<Value>,
}

impl Procedure {
    pub fn new(start: usize, env: Vec<Value>) -> Self {
        Procedure { start, env }
    }
}

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<proc @{}>", self.start)
    }
}

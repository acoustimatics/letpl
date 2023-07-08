use std::fmt;
use std::rc::Rc;

use crate::chunk::Address;
use crate::value::Value;

/// Represents a procedure and its captured environment.
pub struct Procedure {
    pub start: Address,
    pub captures: Rc<Vec<Value>>,
}

impl Procedure {
    pub fn new(start: Address, captures: Vec<Value>) -> Self {
        let captures = Rc::new(captures);
        Self { start, captures }
    }
}

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<proc @{}>", self.start)
    }
}

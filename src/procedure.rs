use std::fmt;

use crate::environment::Environment;

/// Represents a procedure and its captured environment.
#[derive(Clone)]
pub struct Procedure {
    /// The starting index.
    pub start: usize,

    /// The environment when the procedure object was made.
    pub env: Environment,
}

impl Procedure {
    pub fn new(start: usize, env: &Environment) -> Self {
        let env = env.clone();
        Procedure { start, env }
    }
}

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<proc @{}>", self.start)
    }
}

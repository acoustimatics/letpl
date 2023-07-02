use std::fmt;

use crate::environment::Environment;

/// Represents a procedure and its captured environment.
#[derive(Clone)]
pub struct Procedure {
    /// The parameter name.
    pub var: String,

    /// The starting index.
    pub start: usize,

    /// The environment when the procedure object was made.
    pub env: Environment,
}

impl Procedure {
    pub fn new(var: &str, start: usize, env: &Environment) -> Self {
        let var = var.to_owned();
        let env = env.clone();
        Procedure { var, start, env }
    }
}

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<({}) @{}>", self.var, self.start)
    }
}

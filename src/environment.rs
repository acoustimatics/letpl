use std::fmt;

use crate::value::Value;

/// Represents a value bound to a name.
#[derive(Clone)]
struct Binding {
    name: String,
    value: Value,
}

impl Binding {
    fn new(name: &str, value: Value) -> Self {
        let name = name.to_owned();
        Binding { name, value }
    }
}

/// Represents a linked list of bindings.
#[derive(Clone)]
pub struct Environment {
    bindings: Vec<Binding>,
}

impl Environment {
    fn new(bindings: Vec<Binding>) -> Self {
        Environment { bindings }
    }

    /// Creates a new empty environment.
    pub fn empty() -> Self {
        Environment::new(Vec::new())
    }

    pub fn push(&mut self, name: &str, value: Value) {
        let binding = Binding::new(name, value);
        self.bindings.push(binding);
    }

    pub fn pop(&mut self) -> Result<(), String> {
        match self.bindings.pop() {
            Some(_) => Ok(()),
            None => Err(format!("environment underflow")),
        }
    }

    /// Returns the value bound to the lookup name if such a binding exists.
    pub fn fetch(&self, lookup_name: &str) -> Option<&Value> {
        self.bindings
            .iter()
            .rev()
            .find(|b| b.name == lookup_name)
            .map(|b| &b.value)
    }
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for binding in self.bindings.iter() {
            write!(f, "{}:{:?} ", binding.name, binding.value)?;
        }
        write!(f, "}}")
    }
}

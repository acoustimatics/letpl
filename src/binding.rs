use std::fmt;

pub type Scope = usize;

#[derive(Clone)]
struct Binding {
    name: String,
    scope: Scope,
}

impl Binding {
    fn new(name: &str, scope: Scope) -> Self {
        let name = name.to_owned();
        Binding { name, scope }
    }
}

#[derive(Clone)]
pub struct BindingTable {
    bindings: Vec<Binding>,
}

impl BindingTable {
    pub fn new() -> Self {
        let bindings = Vec::new();
        Self { bindings }
    }

    pub fn push(&mut self, name: &str) {
        let scope = self.bindings.len();
        let binding = Binding::new(name, scope);
        self.bindings.push(binding);
    }

    pub fn pop(&mut self) -> Result<(), String> {
        match self.bindings.pop() {
            Some(_) => Ok(()),
            None => Err("environment underflow".to_string()),
        }
    }

    pub fn lookup(&self, lookup_name: &str) -> Option<&Scope> {
        self.bindings
            .iter()
            .rev()
            .find(|b| b.name == lookup_name)
            .map(|b| &b.scope)
    }
}

impl fmt::Debug for BindingTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for binding in self.bindings.iter() {
            write!(f, "{}#{} ", binding.name, binding.scope)?;
        }
        write!(f, "}}")
    }
}

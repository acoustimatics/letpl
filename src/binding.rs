/// Represents a value bound to a name.
struct Binding {
    name: String,
    depth: usize,
}

impl Binding {
    fn new(name: &str, depth: usize) -> Self {
        let name = name.to_owned();
        Binding { name, depth }
    }
}

pub struct BindingTable {
    bindings: Vec<Binding>,
}

impl BindingTable {
    pub fn new() -> Self {
        let bindings = Vec::new();
        Self { bindings }
    }

    pub fn push(&mut self, name: &str) {
        let depth = self.bindings.len();
        let binding = Binding::new(name, depth);
        self.bindings.push(binding);
    }

    pub fn pop(&mut self) -> Result<(), String> {
        match self.bindings.pop() {
            Some(_) => Ok(()),
            None => Err("environment underflow".to_string()),
        }
    }

    pub fn lookup(&self, lookup_name: &str) -> Option<&usize> {
        self.bindings
            .iter()
            .rev()
            .find(|b| b.name == lookup_name)
            .map(|b| &b.depth)
    }
}

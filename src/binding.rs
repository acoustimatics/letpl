use std::fmt;

#[derive(Clone)]
struct Binding {
    name: String,
    stack_index: usize,
}

impl Binding {
    fn new(name: &str, stack_index: usize) -> Self {
        let name = name.to_owned();
        Self { name, stack_index }
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

    pub fn push(&mut self, name: &str, stack_index: usize) {
        let binding = Binding::new(name, stack_index);
        self.bindings.push(binding);
    }

    pub fn pop(&mut self) {
        self.bindings.pop().expect("binding table underflow");
    }

    pub fn lookup(&self, lookup_name: &str) -> Option<usize> {
        self.bindings
            .iter()
            .rev()
            .find(|b| b.name == lookup_name)
            .map(|b| b.stack_index)
    }
}

impl fmt::Debug for BindingTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for binding in self.bindings.iter() {
            write!(f, "{}#{} ", binding.name, binding.stack_index)?;
        }
        write!(f, "}}")
    }
}

//! A table of symbols and associated values.

#[derive(Clone)]
struct Symbol<T: Clone> {
    name: String,
    value: T,
}

impl<T: Clone> Symbol<T> {
    fn new(name: &str, value: &T) -> Self {
        let name = name.to_owned();
        let value = value.clone();
        Self { name, value }
    }
}

#[derive(Clone)]
pub struct SymbolTable<T: Clone> {
    symbols: Vec<Symbol<T>>,
}

impl<T: Clone> SymbolTable<T> {
    pub fn new() -> Self {
        let symbols = Vec::new();
        Self { symbols }
    }

    pub fn push(&mut self, name: &str, value: &T) {
        let symbol = Symbol::new(name, value);
        self.symbols.push(symbol);
    }

    pub fn pop(&mut self) {
        self.symbols.pop().expect("symbol table underflow");
    }

    pub fn lookup(&self, lookup_name: &str) -> Option<&T> {
        self.symbols
            .iter()
            .rev()
            .find(|s| s.name == lookup_name)
            .map(|s| &s.value)
    }
}

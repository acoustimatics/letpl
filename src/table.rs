//! A table of names and associated values.

struct Item<T> {
    name: String,
    value: T,
}

impl<T: Clone> Item<T> {
    fn new(name: String, value: T) -> Self {
        Self { name, value }
    }
}

pub struct Table<T> {
    items: Vec<Item<T>>,
}

impl<T: Clone> Table<T> {
    pub fn new() -> Self {
        let symbols = Vec::new();
        Self { items: symbols }
    }

    pub fn push(&mut self, name: String, value: T) {
        let symbol = Item::new(name, value);
        self.items.push(symbol);
    }

    pub fn pop(&mut self) {
        self.items.pop().expect("symbol table underflow");
    }

    pub fn lookup(&self, name: &str) -> Option<&T> {
        self.items
            .iter()
            .rev()
            .find(|s| s.name == name)
            .map(|s| &s.value)
    }
}

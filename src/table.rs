//! A table of names and associated values.

pub struct Item<T> {
    pub name: String,
    pub value: T,
}

impl<T> Item<T> {
    fn new(name: String, value: T) -> Self {
        Self { name, value }
    }
}

pub struct Table<T> {
    pub items: Vec<Item<T>>,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        let symbols = Vec::new();
        Self { items: symbols }
    }

    pub fn len(&self) -> usize {
        self.items.len()
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
            .find(|item| item.name == name)
            .map(|item| &item.value)
    }

    pub fn lookup_offset(&self, name: &str) -> Option<usize> {
        self.items
            .iter()
            .enumerate()
            .rev()
            .find(|(_, item)| item.name == name)
            .map(|(offset, _)| offset)
    }
}

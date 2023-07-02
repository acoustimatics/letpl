use std::fmt;
use std::rc::Rc;

use crate::value::Value;

/// A node in a linked list of bindings.
struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(item: T, next: &Link<T>) -> Self {
        let next = next.clone();
        Node { item, next }
    }
}

/// A reference to a Node.
enum Link<T> {
    Empty,
    Node(Rc<Node<T>>),
}

impl<T> Link<T> {
    fn from_node(node: Node<T>) -> Self {
        Link::Node(Rc::new(node))
    }

    fn node(&self) -> Option<&Node<T>> {
        match self {
            Link::Empty => None,
            Link::Node(n) => Some(n),
        }
    }

    fn iter(&self) -> LinkIterator<T> {
        LinkIterator::new(self)
    }
}

impl<T> Clone for Link<T> {
    fn clone(&self) -> Self {
        match self {
            Link::Empty => Link::Empty,
            Link::Node(n) => Link::Node(n.clone()),
        }
    }
}

/// Iterator over an Link's items.
struct LinkIterator<'a, T> {
    link: &'a Link<T>,
}

impl<'a, T> LinkIterator<'a, T> {
    fn new(link: &'a Link<T>) -> Self {
        LinkIterator { link }
    }
}

impl<'a, T> Iterator for LinkIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.link.node().map(|n| {
            self.link = &n.next;
            &n.item
        })
    }
}

/// Represents a value bound to a name.
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
pub struct Environment {
    head: Link<Binding>,
}

impl Environment {
    fn new(head: Link<Binding>) -> Self {
        Environment { head }
    }

    /// Creates a new empty environment.
    pub fn empty() -> Self {
        Environment::new(Link::Empty)
    }

    /// Creates a new environment by extending a given environment with a new
    /// binding.
    pub fn extend(&self, name: &str, value: Value) -> Self {
        let binding = Binding::new(name, value);
        let node = Node::new(binding, &self.head);
        let head = Link::from_node(node);
        Environment::new(head)
    }

    /// Creates a new environment with this environments inner environment. It
    /// is the opposite of the extend function.
    pub fn retract(&self) -> Option<Self> {
        self.head.node().map(|n| Environment::new(n.next.clone()))
    }

    /// Returns the value bound to the lookup name if such a binding exists.
    pub fn apply(&self, lookup_name: &str) -> Option<&Value> {
        self.head
            .iter()
            .find(|b| b.name == lookup_name)
            .map(|b| &b.value)
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Environment::new(self.head.clone())
    }
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for binding in self.head.iter() {
            write!(f, "{}:{:?} ", binding.name, binding.value)?;
        }
        write!(f, "}}")
    }
}

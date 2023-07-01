use std::fmt;
use std::rc::Rc;

/// Represents a value bound to a name.
struct Binding<T> {
    name: String,
    value: T,
}

impl<T> Binding<T> {
    fn new(name: &str, value: T) -> Binding<T> {
        let name = name.to_owned();
        Binding { name, value }
    }
}

/// A reference to a Node.
enum Link<T> {
    Empty,
    Node(Rc<Node<T>>),
}

impl<T> Link<T> {
    fn from_node(node: Node<T>) -> Link<T> {
        Link::Node(Rc::new(node))
    }

    fn node(&self) -> Option<&Node<T>> {
        match self {
            Link::Empty => None,
            Link::Node(n) => Some(n),
        }
    }
}

impl<T> Clone for Link<T> {
    fn clone(&self) -> Link<T> {
        match self {
            Link::Empty => Link::Empty,
            Link::Node(n) => Link::Node(n.clone()),
        }
    }
}

/// A node in a linked list of bindings.
struct Node<T> {
    binding: Binding<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(binding: Binding<T>, next: &Link<T>) -> Node<T> {
        let next = next.clone();
        Node { binding, next }
    }
}

/// Represents a linked list of bindings.
pub struct Environment<T> {
    head: Link<T>,
}

impl<T> Environment<T> {
    fn new(head: Link<T>) -> Self {
        Environment { head }
    }

    /// Creates a new empty environment.
    pub fn empty() -> Self {
        Environment::new(Link::Empty)
    }

    /// Creates a new environment by extending a given environment with a new
    /// binding.
    pub fn extend(&self, name: &str, value: T) -> Self {
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
    pub fn apply(&self, lookup_name: &str) -> Option<&T> {
        self.iter()
            .find(|b| b.name == lookup_name)
            .map(|b| &b.value)
    }

    fn iter(&self) -> EnvironmentIterator<T> {
        EnvironmentIterator::new(&self.head)
    }
}

impl<T> Clone for Environment<T> {
    fn clone(&self) -> Self {
        Environment::new(self.head.clone())
    }
}

impl<T: fmt::Debug> fmt::Debug for Environment<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for binding in self.iter() {
            write!(f, "{}:{:?} ", binding.name, binding.value)?;
        }
        write!(f, "}}")
    }
}

/// Iterator over an Environment's bindings.
struct EnvironmentIterator<'a, T> {
    link: &'a Link<T>,
}

impl<'a, T> EnvironmentIterator<'a, T> {
    fn new(link: &'a Link<T>) -> Self {
        EnvironmentIterator { link }
    }
}

impl<'a, T> Iterator for EnvironmentIterator<'a, T> {
    type Item = &'a Binding<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.link.node().map(|n| {
            self.link = &n.next;
            &n.binding
        })
    }
}

use std::rc::Rc;

enum Node<T> {
    Empty,
    Extended(String, T, Rc<Node<T>>),
}

impl<T> Node<T> {
    fn apply(&self, lookup_name: &str) -> Option<&T> {
        match self {
            Node::Empty => None,
            Node::Extended(node_name, value, _) if node_name.eq(lookup_name) => Some(value),
            Node::Extended(_, _, extended) => extended.apply(lookup_name),
        }
    }
}

/// Represents an environment in which identifiers are bound to values.
pub struct Environment<T> {
    head: Rc<Node<T>>,
}

impl<T> Environment<T> {
    /// Creates a new empty environment.
    pub fn empty() -> Environment<T> {
        Environment {
            head: Rc::new(Node::Empty),
        }
    }

    /// Creates a new environment by extending a given environment with a new
    /// bound identifier.
    pub fn extend(&self, name: String, value: T) -> Environment<T> {
        let extended = Rc::clone(&self.head);
        Environment {
            head: Rc::new(Node::Extended(name, value, extended)),
        }
    }

    /// Creates a new environment with this environments inner environment. It
    /// is the opposite of the extend function.
    pub fn retract(&self) -> Option<Environment<T>> {
        match self.head.as_ref() {
            Node::Empty => None,
            Node::Extended(_, _, head) => {
                let head = Rc::clone(head);
                Some(Environment { head })
            }
        }
    }

    /// Returns the value bound to the given identifier if such a binding
    /// exists.
    pub fn apply(&self, lookup_name: &str) -> Option<&T> {
        self.head.apply(lookup_name)
    }
}

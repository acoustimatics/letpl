//! Abstract syntax tree types for letpl.

use std::fmt;
use std::rc::Rc;

/// A program node in an AST.
pub struct Program {
    /// The program's expression.
    pub expr: Box<Expr>,
}

/// An expression node in an AST.
pub enum Expr {
    /// An expression guarded by a test expression.
    Assert {
        line: usize,
        test: Box<Expr>,
        body: Box<Expr>,
    },

    /// A procedure call expression.
    Call { proc: Box<Expr>, arg: Box<Expr> },

    /// A conditional expression.
    If {
        test: Box<Expr>,
        consequent: Box<Expr>,
        alternate: Box<Expr>,
    },

    /// An expression that test if a sub-expression is zero.
    IsZero(Box<Expr>),

    /// An expression with a name bound to a value.
    Let {
        name: String,
        expr: Box<Expr>,
        body: Box<Expr>,
    },

    /// A recursive procedure definition expression.
    LetRec {
        t_result: Type,
        name: String,
        param: Param,
        proc_body: Box<Expr>,
        let_body: Box<Expr>,
    },

    /// A literal Boolean expression.
    LiteralBool(bool),

    /// A literal integer expression.
    LiteralInt(i64),

    /// A name lookup expression.
    Name(String),

    /// A procedure definition expression.
    Proc { param: Param, body: Box<Expr> },

    /// An expression that subtracts right from left.
    Subtract { left: Box<Expr>, right: Box<Expr> },
}

pub struct Param {
    pub name: String,
    pub t: Type,
}

impl Param {
    pub fn new(name: String, t: Type) -> Param {
        Param { name, t }
    }
}

pub struct Type {
    tag: Rc<TypeTag>,
}

impl Type {
    pub fn new_int() -> Self {
        let tag = Rc::new(TypeTag::Int);
        Self { tag }
    }

    pub fn new_bool() -> Self {
        let tag = Rc::new(TypeTag::Bool);
        Self { tag }
    }

    pub fn new_proc(t_param: Type, t_result: Type) -> Self {
        let tag = Rc::new(TypeTag::Proc { t_param, t_result });
        Self { tag }
    }

    pub fn is_int(&self) -> bool {
        match self.tag.as_ref() {
            TypeTag::Int => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self.tag.as_ref() {
            TypeTag::Bool => true,
            _ => false,
        }
    }

    pub fn as_proc(&self) -> Option<(&Type, &Type)> {
        match self.tag.as_ref() {
            TypeTag::Proc { t_param, t_result } => Some((t_param, t_result)),
            _ => None,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.tag.as_ref() == other.tag.as_ref()
    }
}

impl Clone for Type {
    fn clone(&self) -> Self {
        let tag = Rc::clone(&self.tag);
        Self { tag }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag)
    }
}

enum TypeTag {
    Int,
    Bool,
    Proc { t_param: Type, t_result: Type },
}

impl PartialEq for TypeTag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeTag::Int, TypeTag::Int) | (TypeTag::Bool, TypeTag::Bool) => true,
            (
                TypeTag::Proc {
                    t_param: t_param_left,
                    t_result: t_result_left,
                },
                TypeTag::Proc {
                    t_param: t_param_right,
                    t_result: t_result_right,
                },
            ) => t_param_left == t_param_right && t_result_left == t_result_right,
            _ => false,
        }
    }
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeTag::Int => write!(f, "int"),
            TypeTag::Bool => write!(f, "bool"),
            TypeTag::Proc { t_param, t_result } => write!(f, "({t_param} -> {t_result})"),
        }
    }
}

pub mod nameless {
    //! A namless version of the AST, that is, an AST without identifiers.

    pub struct Program {
        pub expr: Box<Expr>,
    }

    pub enum Expr {
        /// An expression which guards its body expression by a test expression.
        Assert {
            line: usize,
            test: Box<Expr>,
            body: Box<Expr>,
        },

        /// A procedure call expression.
        Call {
            proc: Box<Expr>,
            arg: Box<Expr>,
        },

        Capture(usize),

        Global(usize),

        /// A conditional expression.
        If {
            test: Box<Expr>,
            consequent: Box<Expr>,
            alternate: Box<Expr>,
        },

        IsZero(Box<Expr>),

        Let {
            expr: Box<Expr>,
            body: Box<Expr>,
        },

        LiteralBool(bool),

        /// A literal integer expression.
        LiteralInt(i64),

        Local(usize),

        Proc {
            body: Box<Expr>,
            captures: Vec<Capture>,
        },

        /// An expression that subtracts right from left.
        Subtract {
            left: Box<Expr>,
            right: Box<Expr>,
        },
    }

    pub enum Capture {
        Local(usize),
        Capture(usize),
    }
}

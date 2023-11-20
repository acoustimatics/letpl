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
        guard: Box<Expr>,
        body: Box<Expr>,
    },

    /// A constant numerical expression.
    Const(i64),

    /// An expression that subtracts two sub-expressions.
    Diff(Box<Expr>, Box<Expr>),

    /// An expression that test if a sub-expression is zero.
    IsZero(Box<Expr>),

    /// A conditional expression.
    If(Box<Expr>, Box<Expr>, Box<Expr>),

    /// A variable lookup expression.
    Var(String),

    /// An expression with a name bound to a value.
    Let(String, Box<Expr>, Box<Expr>),

    /// A literal Boolean expression.
    LiteralBool(bool),

    /// A procedure definition expression.
    Proc(String, LetType, Box<Expr>),

    /// A procedure call expression.
    Call(Box<Expr>, Box<Expr>),

    /// A recursive procedure definition expression.
    LetRec {
        result_type: LetType,
        name: String,
        var: String,
        var_type: LetType,
        proc_body: Box<Expr>,
        let_body: Box<Expr>,
    },
}

/// Represents a type in letpl.
pub struct LetType {
    let_type: Rc<TypeTag>,
}

impl LetType {
    pub fn new_int() -> Self {
        let let_type = Rc::new(TypeTag::Int);
        Self { let_type }
    }

    pub fn new_bool() -> Self {
        let let_type = Rc::new(TypeTag::Bool);
        Self { let_type }
    }

    pub fn new_proc(var_type: LetType, result_type: LetType) -> Self {
        let let_type = Rc::new(TypeTag::Proc(var_type, result_type));
        Self { let_type }
    }

    pub fn is_int(&self) -> bool {
        match self.let_type.as_ref() {
            TypeTag::Int => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self.let_type.as_ref() {
            TypeTag::Bool => true,
            _ => false,
        }
    }

    pub fn as_proc(&self) -> Option<(&LetType, &LetType)> {
        match self.let_type.as_ref() {
            TypeTag::Proc(t_arg, t_body) => Some((t_arg, t_body)),
            _ => None,
        }
    }
}

impl PartialEq for LetType {
    fn eq(&self, other: &Self) -> bool {
        self.let_type.as_ref() == other.let_type.as_ref()
    }
}

impl Clone for LetType {
    fn clone(&self) -> Self {
        let let_type = Rc::clone(&self.let_type);
        Self { let_type }
    }
}

impl fmt::Display for LetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.let_type)
    }
}

enum TypeTag {
    Int,
    Bool,
    Proc(LetType, LetType),
}

impl PartialEq for TypeTag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeTag::Int, TypeTag::Int) | (TypeTag::Bool, TypeTag::Bool) => true,
            (TypeTag::Proc(v1, r1), TypeTag::Proc(v2, r2)) => v1 == v2 && r1 == r2,
            _ => false,
        }
    }
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeTag::Int => write!(f, "int"),
            TypeTag::Bool => write!(f, "bool"),
            TypeTag::Proc(var, result) => write!(f, "({var} -> {result})"),
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
            guard: Box<Expr>,
            body: Box<Expr>,
        },

        Call(Box<Expr>, Box<Expr>),

        Capture(usize),

        Const(i64),

        Diff(Box<Expr>, Box<Expr>),

        Global(usize),

        IsZero(Box<Expr>),

        If(Box<Expr>, Box<Expr>, Box<Expr>),

        Let(Box<Expr>, Box<Expr>),

        LiteralBool(bool),

        Local(usize),

        Proc(Box<Expr>, Vec<Capture>),
    }

    pub enum Capture {
        Local(usize),
        Capture(usize),
    }
}

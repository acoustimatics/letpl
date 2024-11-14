//! Abstract syntax tree types for letpl.

use crate::types::Type;

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

    /// An expression that negates its inner expression.
    Negate(Box<Expr>),

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

pub mod nameless {
    //! A namless version of the AST, that is, an AST without identifiers.
    use crate::offset::{Capture, CaptureOffset, StackOffset};

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

        Capture(CaptureOffset),

        Global(StackOffset),

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

        Local(StackOffset),

        /// An expression that negates its inner expression.
        Negate(Box<Expr>),

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
}

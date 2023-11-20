//! Abstract syntax tree types for letpl.

use crate::types::LetType;

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

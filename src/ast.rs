//! Abstract syntax tree types for letpl.

use crate::types::LetType;

/// Represents a Program node in an AST.
pub struct Program {
    pub expr: Box<Expr>,
}

/// Represents an Expression node in an AST.
pub enum Expr {
    Assert {
        line: usize,
        guard: Box<Expr>,
        body: Box<Expr>,
    },

    /// Represents a constant numerical expression.
    Const(i64),

    /// Represents an expression that takes the difference of two
    /// sub-expressions.
    Diff(Box<Expr>, Box<Expr>),

    /// Represents an expression that test if a sub-expression is zero.
    IsZero(Box<Expr>),

    /// Represents and if/then/else expression.
    If(Box<Expr>, Box<Expr>, Box<Expr>),

    /// Represents a variable lookup expression.
    Var(String),

    /// Represent an let/in expression.
    Let(String, Box<Expr>, Box<Expr>),

    /// Represents a procedure.
    Proc(String, LetType, Box<Expr>),

    /// Represents a procedure call.
    Call(Box<Expr>, Box<Expr>),

    /// Represents a recursve procedure.
    LetRec {
        result_type: LetType,
        name: String,
        var: String,
        var_type: LetType,
        proc_body: Box<Expr>,
        let_body: Box<Expr>,
    },
}

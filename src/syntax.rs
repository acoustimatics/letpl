/// Represents a Program node in an AST.
pub struct Program {
    pub expr: Box<Expr>,
}

/// Represents an Expression node in an AST.
pub enum Expr {
    /// Represents a constant numerical expression.
    Const(f64),

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

    /// Represents a numerical negation.
    Minus(Box<Expr>),

    /// Represents a procedure.
    Proc(String, Box<Expr>),

    /// Represents a procedure call.
    Call(Box<Expr>, Box<Expr>),

    /// Represents a recursve procedure.
    LetRec {
        name: String,
        var: String,
        proc_body: Box<Expr>,
        let_body: Box<Expr>,
    },
}

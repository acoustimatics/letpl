/// Representsa Program node in an AST.
#[derive(Debug)]
pub struct Program {
    pub expr: Box<Expr>,
}

/// Represents an Expression node in an AST.
#[derive(Debug)]
pub enum Expr {
    Const(f64),
    Diff(Box<Expr>, Box<Expr>),
    IsZero(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Var(String),
    Let(String, Box<Expr>, Box<Expr>),
}

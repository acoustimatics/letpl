use std::fmt;

/// Represents a token in a source text.
#[derive(PartialEq)]
pub enum Token {
    Comma,
    Else,
    Eof,
    Equal,
    Identifier(String),
    If,
    In,
    LeftParen,
    Let,
    LetRec,
    Proc,
    Minus,
    MinusSign,
    Number(f64),
    RightParen,
    Then,
    IsZero,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            Token::Comma => ",",
            Token::Else => "else",
            Token::Eof => "EOF",
            Token::Equal => "=",
            Token::Identifier(id) => {
                return write!(f, "identifier({})", id);
            }
            Token::If => "if",
            Token::In => "in",
            Token::LeftParen => "(",
            Token::Let => "let",
            Token::LetRec => "letrec",
            Token::Proc => "proc",
            Token::Minus => "minus",
            Token::MinusSign => "-",
            Token::Number(_) => "number",
            Token::RightParen => ")",
            Token::Then => "then",
            Token::IsZero => "zero?",
        };
        write!(f, "{}", token_str)
    }
}

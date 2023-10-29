//! Lexical analysis for letpl.

use std::fmt;
use std::str::Chars;

/// Represents a token in a source text.
#[derive(PartialEq)]
pub enum Token {
    Arrow,
    Bool,
    Colon,
    Comma,
    Else,
    Eof,
    Equal,
    Identifier(String),
    If,
    In,
    Int,
    LeftParen,
    Let,
    LetRec,
    Proc,
    MinusSign,
    Number(i64),
    RightParen,
    Then,
    IsZero,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            Token::Arrow => "->",
            Token::Bool => "bool",
            Token::Colon => ":",
            Token::Comma => ",",
            Token::Else => "else",
            Token::Eof => "EOF",
            Token::Equal => "=",
            Token::Identifier(id) => {
                return write!(f, "identifier({id})");
            }
            Token::If => "if",
            Token::In => "in",
            Token::Int => "int",
            Token::LeftParen => "(",
            Token::Let => "let",
            Token::LetRec => "letrec",
            Token::Proc => "proc",
            Token::MinusSign => "-",
            Token::Number(_) => "number",
            Token::RightParen => ")",
            Token::Then => "then",
            Token::IsZero => "zero?",
        };
        write!(f, "{token_str}")
    }
}

/// Represents an object which converts a source text into a stream of tokens.
pub struct Scanner<'a> {
    chars: Chars<'a>,
    current: Option<char>,
}

impl<'a> Scanner<'a> {
    /// Creates a scanner object which is ready to produce tokens from a given
    /// source text.
    pub fn new(src: &str) -> Scanner {
        let mut scanner = Scanner {
            chars: src.chars(),
            current: None,
        };
        scanner.advance();
        scanner
    }

    fn advance(&mut self) {
        self.current = self.chars.next();
    }

    fn skip_whitespace_comments(&mut self) {
        let mut in_comment = false;
        while let Some(c) = self.current {
            if c == '#' {
                in_comment = true;
                self.advance();
            } else if c == '\n' && in_comment {
                in_comment = false;
                self.advance();
            } else if is_whitespace(c) || in_comment {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Attempt to get the next token in the source text.
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_comments();

        // Handle end of code.
        if self.current.is_none() {
            return Ok(Token::Eof);
        }

        // Handle identifiers and keywords.
        if self.current.map_or(false, is_alpha) {
            let mut s = String::new();
            while self
                .current
                .map_or(false, |c| is_alpha(c) || is_digit(c) || c == '?')
            {
                self.collect(&mut s);
            }

            let token = match s.as_ref() {
                "bool" => Token::Bool,
                "else" => Token::Else,
                "if" => Token::If,
                "in" => Token::In,
                "int" => Token::Int,
                "let" => Token::Let,
                "letrec" => Token::LetRec,
                "proc" => Token::Proc,
                "then" => Token::Then,
                "zero?" => Token::IsZero,
                _ => Token::Identifier(s),
            };

            return Ok(token);
        }

        // Handle a number literal.
        if self.current.map_or(false, is_digit) {
            return self.number_literal();
        }

        // Handle operators.
        let token = match self.current.unwrap() {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '-' => Token::MinusSign,
            '=' => Token::Equal,
            c => return Err(format!("unexpected character '{c}'")),
        };

        // Advance past the last character in the operator.
        self.advance();

        // Check for two character operators.
        let token = match self.current {
            Some('>') if token == Token::MinusSign => {
                self.advance();
                Token::Arrow
            }
            _ => token,
        };

        Ok(token)
    }

    fn number_literal(&mut self) -> Result<Token, String> {
        let mut s = String::new();

        while self.current.map_or(false, is_digit) {
            self.collect(&mut s);
        }

        match s.parse() {
            Ok(x) => Ok(Token::Number(x)),
            Err(_) => Err(format!("'{s}' cannot be converted to a number")),
        }
    }

    fn collect(&mut self, s: &mut String) {
        s.push(self.current.unwrap());
        self.advance();
    }
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

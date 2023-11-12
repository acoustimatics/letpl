//! Lexical analysis for letpl.

use std::fmt;
use std::str::Chars;

/// Represents a token's type in a source text.
#[derive(PartialEq)]
pub enum TokenTag {
    Arrow,
    Assert,
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

impl fmt::Display for TokenTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            TokenTag::Arrow => "->",
            TokenTag::Assert => "assert",
            TokenTag::Bool => "bool",
            TokenTag::Colon => ":",
            TokenTag::Comma => ",",
            TokenTag::Else => "else",
            TokenTag::Eof => "EOF",
            TokenTag::Equal => "=",
            TokenTag::Identifier(id) => {
                return write!(f, "identifier({id})");
            }
            TokenTag::If => "if",
            TokenTag::In => "in",
            TokenTag::Int => "int",
            TokenTag::LeftParen => "(",
            TokenTag::Let => "let",
            TokenTag::LetRec => "letrec",
            TokenTag::Proc => "proc",
            TokenTag::MinusSign => "-",
            TokenTag::Number(_) => "number",
            TokenTag::RightParen => ")",
            TokenTag::Then => "then",
            TokenTag::IsZero => "zero?",
        };
        write!(f, "{token_str}")
    }
}

/// A token from a source text.
pub struct Token {
    /// The token's type.
    pub tag: TokenTag,

    /// The line in the source text on which the token starts.
    pub line: usize,
}

impl Token {
    /// A token constructor function.
    pub fn new(tag: TokenTag, line: usize) -> Self {
        Self { tag, line }
    }
}

/// Represents an object which converts a source text into a stream of tokens.
pub struct Scanner<'a> {
    chars: Chars<'a>,
    current: Option<char>,
    line: usize,
}

impl<'a> Scanner<'a> {
    /// Creates a scanner object which is ready to produce tokens from a given
    /// source text.
    pub fn new(src: &str) -> Scanner {
        let mut scanner = Scanner {
            chars: src.chars(),
            current: None,
            line: 1,
        };
        scanner.advance();
        scanner
    }

    fn advance(&mut self) {
        if let Some('\n') = self.current {
            self.line += 1;
        }
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

        if self.current.is_none() {
            Ok(Token::new(TokenTag::Eof, self.line))
        } else if self.current.map_or(false, is_alpha) {
            self.identifier()
        } else if self.current.map_or(false, is_digit) {
            self.number_literal()
        } else {
            self.symbol()
        }
    }

    fn identifier(&mut self) -> Result<Token, String> {
        let line = self.line;

        let mut s = String::new();
        while self
            .current
            .map_or(false, |c| is_alpha(c) || is_digit(c) || c == '?')
        {
            self.collect(&mut s);
        }

        let tag = match s.as_ref() {
            "assert" => TokenTag::Assert,
            "bool" => TokenTag::Bool,
            "else" => TokenTag::Else,
            "if" => TokenTag::If,
            "in" => TokenTag::In,
            "int" => TokenTag::Int,
            "let" => TokenTag::Let,
            "letrec" => TokenTag::LetRec,
            "proc" => TokenTag::Proc,
            "then" => TokenTag::Then,
            "zero?" => TokenTag::IsZero,
            _ => TokenTag::Identifier(s),
        };

        return Ok(Token::new(tag, line));
    }

    fn number_literal(&mut self) -> Result<Token, String> {
        let line = self.line;

        let mut s = String::new();

        while self.current.map_or(false, is_digit) {
            self.collect(&mut s);
        }

        match s.parse() {
            Ok(x) => Ok(Token::new(TokenTag::Number(x), line)),
            Err(_) => Err(format!("'{s}' cannot be converted to a number")),
        }
    }

    fn symbol(&mut self) -> Result<Token, String> {
        let line = self.line;

        // Handle operators.
        let tag = match self.current.unwrap() {
            '(' => TokenTag::LeftParen,
            ')' => TokenTag::RightParen,
            ':' => TokenTag::Colon,
            ',' => TokenTag::Comma,
            '-' => TokenTag::MinusSign,
            '=' => TokenTag::Equal,
            c => return Err(format!("unexpected character '{c}'")),
        };

        // Advance past the last character in the operator.
        self.advance();

        // Check for two character operators.
        let tag = match self.current {
            Some('>') if tag == TokenTag::MinusSign => {
                self.advance();
                TokenTag::Arrow
            }
            _ => tag,
        };

        Ok(Token::new(tag, line))
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

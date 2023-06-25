use crate::token::Token;
use std::str::Chars;

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

    /// Attempt to get the next token in the source text.
    pub fn next_token(&mut self) -> Result<Token, String> {
        use Token::*;

        // Skip whitespace.
        loop {
            match self.current {
                Some(c) if is_whitespace(c) => self.advance(),
                _ => break,
            }
        }

        // Handle end of code.
        if self.current.is_none() {
            return Ok(Eof);
        }

        // Handle identifiers and keywords.
        if self.current.map_or(false, |c| is_alpha(c)) {
            let mut s = String::new();
            while self
                .current
                .map_or(false, |c| is_alpha(c) || is_digit(c) || c == '?')
            {
                s.push(self.current.unwrap());
                self.advance();
            }

            let token = match s.as_ref() {
                "else" => Else,
                "if" => If,
                "in" => In,
                "let" => Let,
                "then" => Then,
                "zero?" => IsZero,
                _ => Identifier(s),
            };

            return Ok(token);
        }

        // Handle a number literal.
        if self.current.map_or(false, |c| is_digit(c)) {
            let mut s = String::new();

            while self.current.map_or(false, |c| is_digit(c)) {
                s.push(self.current.unwrap());
                self.advance();
            }

            return match s.parse() {
                Ok(x) => Ok(Number(x)),
                Err(_) => Err(format!("'{}' cannot be converted to a number", s)),
            };
        }

        // Handle operators.
        let token = match self.current.unwrap() {
            '(' => LeftParen,
            ')' => RightParen,
            ',' => Comma,
            '-' => Minus,
            '=' => Equal,
            c => return Err(format!("unexpected character '{}'", c)),
        };

        // Advance past the last character in the operator.
        self.advance();

        return Ok(token);
    }
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

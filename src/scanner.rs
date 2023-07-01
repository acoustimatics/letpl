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
                self.collect(&mut s);
            }

            let token = match s.as_ref() {
                "else" => Else,
                "if" => If,
                "in" => In,
                "let" => Let,
                "proc" => Proc,
                "minus" => Minus,
                "then" => Then,
                "zero?" => IsZero,
                _ => Identifier(s),
            };

            return Ok(token);
        }

        // Handle a number literal.
        if self.current.map_or(false, |c| is_digit(c)) {
            return self.number_literal();
        }

        // Handle operators.
        let token = match self.current.unwrap() {
            '(' => LeftParen,
            ')' => RightParen,
            ',' => Comma,
            '-' => MinusSign,
            '=' => Equal,
            c => return Err(format!("unexpected character '{}'", c)),
        };

        // Advance past the last character in the operator.
        self.advance();

        return Ok(token);
    }

    fn number_literal(&mut self) -> Result<Token, String> {
        let mut s = String::new();

        while self.current.map_or(false, |c| is_digit(c)) {
            self.collect(&mut s);
        }

        if self.current.map_or(false, |c| c == '.') {
            self.collect(&mut s);

            match self.current {
                Some(c) if is_digit(c) => self.collect(&mut s),
                _ => {
                    return Err(String::from("expected digit after decimal point"));
                }
            }

            while self.current.map_or(false, |c| is_digit(c)) {
                self.collect(&mut s)
            }
        }

        match s.parse() {
            Ok(x) => Ok(Token::Number(x)),
            Err(_) => Err(format!("'{}' cannot be converted to a number", s)),
        }
    }

    fn collect(&mut self, s: &mut String) {
        s.push(self.current.unwrap());
        self.advance();
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

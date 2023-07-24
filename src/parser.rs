use std::fmt;
use std::str::Chars;

pub struct LetType {
    let_type: Box<TypeTag>,
}

impl LetType {
    fn new_int() -> Self {
        let let_type = Box::new(TypeTag::Int);
        Self { let_type }
    }

    fn new_bool() -> Self {
        let let_type = Box::new(TypeTag::Bool);
        Self { let_type }
    }

    fn new_proc(var_type: LetType, result_type: LetType) -> Self {
        let let_type = Box::new(TypeTag::Proc(var_type, result_type));
        Self { let_type }
    }
}

pub enum TypeTag {
    Int,
    Bool,
    Proc(LetType, LetType),
}

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

    Print(Box<Expr>),

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

/// Represents a token in a source text.
#[derive(PartialEq)]
enum Token {
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
    Print,
    Proc,
    MinusSign,
    Number(f64),
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
                return write!(f, "identifier({})", id);
            }
            Token::If => "if",
            Token::In => "in",
            Token::Int => "int",
            Token::LeftParen => "(",
            Token::Let => "let",
            Token::LetRec => "letrec",
            Token::Print => "print",
            Token::Proc => "proc",
            Token::MinusSign => "-",
            Token::Number(_) => "number",
            Token::RightParen => ")",
            Token::Then => "then",
            Token::IsZero => "zero?",
        };
        write!(f, "{}", token_str)
    }
}

/// Represents an object which converts a source text into a stream of tokens.
struct Scanner<'a> {
    chars: Chars<'a>,
    current: Option<char>,
}

impl<'a> Scanner<'a> {
    /// Creates a scanner object which is ready to produce tokens from a given
    /// source text.
    fn new(src: &str) -> Scanner {
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
    fn next_token(&mut self) -> Result<Token, String> {
        use Token::*;

        self.skip_whitespace_comments();

        // Handle end of code.
        if self.current.is_none() {
            return Ok(Eof);
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
                "bool" => Bool,
                "else" => Else,
                "if" => If,
                "in" => In,
                "int" => Int,
                "let" => Let,
                "letrec" => LetRec,
                "print" => Print,
                "proc" => Proc,
                "then" => Then,
                "zero?" => IsZero,
                _ => Identifier(s),
            };

            return Ok(token);
        }

        // Handle a number literal.
        if self.current.map_or(false, is_digit) {
            return self.number_literal();
        }

        // Handle operators.
        let token = match self.current.unwrap() {
            '(' => LeftParen,
            ')' => RightParen,
            ':' => Colon,
            ',' => Comma,
            '-' => MinusSign,
            '=' => Equal,
            c => return Err(format!("unexpected character '{}'", c)),
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

        if self.current.map_or(false, |c| c == '.') {
            self.collect(&mut s);

            match self.current {
                Some(c) if is_digit(c) => self.collect(&mut s),
                _ => {
                    return Err(String::from("expected digit after decimal point"));
                }
            }

            while self.current.map_or(false, is_digit) {
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
    c.is_ascii_alphabetic() || c == '_'
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

type ExprResult = Result<Box<Expr>, String>;

/// Parses a given source text, giving an AST representing the program.
pub fn parse(src: &str) -> Result<Program, String> {
    let mut parser = Parser::new(src)?;
    parser.program()
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    fn new(src: &str) -> Result<Parser, String> {
        let mut scanner = Scanner::new(src);
        let current = scanner.next_token()?;
        Ok(Parser { scanner, current })
    }

    fn advance(&mut self) -> Result<(), String> {
        self.current = self.scanner.next_token()?;
        Ok(())
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current == expected {
            self.advance()?;
            Ok(())
        } else {
            let message = format!("expected `{:}` but got `{:}`", expected, self.current);
            Err(message)
        }
    }

    fn expect_identifer(&mut self) -> Result<String, String> {
        if let Token::Identifier(name) = &self.current {
            let name = name.clone();
            self.advance()?;
            Ok(name)
        } else {
            let msg = format!("expected identifier but found {:}", self.current);
            Err(msg)
        }
    }

    fn program(&mut self) -> Result<Program, String> {
        let expr = self.expr()?;
        self.expect(Token::Eof)?;
        Ok(Program { expr })
    }

    fn expr(&mut self) -> ExprResult {
        match &self.current {
            Token::Number(x) => {
                let x = *x;
                self.advance()?;
                Ok(Box::new(Expr::Const(x)))
            }
            Token::MinusSign => self.diff(),
            Token::IsZero => self.is_zero(),
            Token::If => self.if_expr(),
            Token::Identifier(var) => {
                let var = var.clone();
                self.advance()?;
                Ok(Box::new(Expr::Var(var)))
            }
            Token::Let => self.let_expr(),
            Token::LetRec => self.let_rec_expr(),
            Token::Print => self.print_expr(),
            Token::Proc => self.proc_expr(),
            Token::LeftParen => self.call_expr(),
            unexpected_token => Err(format!("unexpected token `{:}`", unexpected_token)),
        }
    }

    fn diff(&mut self) -> ExprResult {
        self.advance()?;
        self.expect(Token::LeftParen)?;
        let left_expr = self.expr()?;
        self.expect(Token::Comma)?;
        let right_expr = self.expr()?;
        self.expect(Token::RightParen)?;

        Ok(Box::new(Expr::Diff(left_expr, right_expr)))
    }

    fn is_zero(&mut self) -> ExprResult {
        self.advance()?;
        self.expect(Token::LeftParen)?;
        let expr = self.expr()?;
        self.expect(Token::RightParen)?;

        Ok(Box::new(Expr::IsZero(expr)))
    }

    fn if_expr(&mut self) -> ExprResult {
        self.advance()?;
        let condition = self.expr()?;
        self.expect(Token::Then)?;
        let consequence = self.expr()?;
        self.expect(Token::Else)?;
        let alternative = self.expr()?;

        Ok(Box::new(Expr::If(condition, consequence, alternative)))
    }

    fn let_expr(&mut self) -> ExprResult {
        self.advance()?;
        let var = self.expect_identifer()?;
        self.expect(Token::Equal)?;
        let expr = self.expr()?;
        self.expect(Token::In)?;
        let body = self.expr()?;

        Ok(Box::new(Expr::Let(var, expr, body)))
    }

    fn let_rec_expr(&mut self) -> ExprResult {
        self.advance()?;
        let name = self.expect_identifer()?;
        self.expect(Token::LeftParen)?;
        let var = self.expect_identifer()?;
        self.expect(Token::Colon)?;
        let var_type = self.parse_type()?;
        self.expect(Token::RightParen)?;
        self.expect(Token::Arrow)?;
        let result_type = self.parse_type()?;
        self.expect(Token::Equal)?;
        let proc_body = self.expr()?;
        self.expect(Token::In)?;
        let let_body = self.expr()?;

        Ok(Box::new(Expr::LetRec {
            result_type,
            name,
            var,
            var_type,
            proc_body,
            let_body,
        }))
    }

    fn print_expr(&mut self) -> ExprResult {
        self.advance()?;
        self.expect(Token::LeftParen)?;
        let expr = self.expr()?;
        self.expect(Token::RightParen)?;

        Ok(Box::new(Expr::Print(expr)))
    }

    fn proc_expr(&mut self) -> ExprResult {
        self.advance()?;
        self.expect(Token::LeftParen)?;
        let var = self.expect_identifer()?;
        self.expect(Token::Colon)?;
        let ty = self.parse_type()?;
        self.expect(Token::RightParen)?;
        let body = self.expr()?;

        Ok(Box::new(Expr::Proc(var, ty, body)))
    }

    fn call_expr(&mut self) -> ExprResult {
        self.advance()?;
        let operator = self.expr()?;
        let operand = self.expr()?;
        self.expect(Token::RightParen)?;

        Ok(Box::new(Expr::Call(operator, operand)))
    }

    fn parse_type(&mut self) -> Result<LetType, String> {
        match self.current {
            Token::Int =>  {
                self.advance()?;
                Ok(LetType::new_int())
            }
            Token::Bool => {
                self.advance()?;
                Ok(LetType::new_bool())
            }
            Token::LeftParen => {
                self.advance()?;
                let var_type = self.parse_type()?;
                self.expect(Token::Arrow)?;
                let result_type = self.parse_type()?;
                self.expect(Token::RightParen)?;
                Ok(LetType::new_proc(var_type, result_type))
            }
            _ => Err(format!("unexpected token `{}`", self.current)),
        }
    }
}

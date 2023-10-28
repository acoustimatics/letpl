//! A recursive decent letpl parser.

use crate::ast::{Program, Expr};
use crate::types::LetType;
use crate::scanner::{Token, Scanner};

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
            Token::Proc => self.proc_expr(),
            Token::LeftParen => self.call_expr(),
            unexpected_token => Err(format!("unexpected token `{unexpected_token:}`")),
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
            Token::Int => {
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

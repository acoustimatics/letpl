//! A recursive decent letpl parser.

use crate::ast::{Expr, LetType, Program};
use crate::scanner::{Scanner, Token, TokenTag};

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

    fn expect(&mut self, expected: TokenTag) -> Result<(), String> {
        if self.current.tag == expected {
            self.advance()?;
            Ok(())
        } else {
            let message = format!("expected `{:}` but got `{:}`", expected, self.current.tag);
            Err(message)
        }
    }

    fn expect_identifer(&mut self) -> Result<String, String> {
        if let TokenTag::Identifier(name) = &self.current.tag {
            let name = name.clone();
            self.advance()?;
            Ok(name)
        } else {
            let msg = format!("expected identifier but found {:}", self.current.tag);
            Err(msg)
        }
    }

    fn program(&mut self) -> Result<Program, String> {
        let expr = self.expr()?;
        self.expect(TokenTag::Eof)?;
        Ok(Program { expr })
    }

    fn expr(&mut self) -> ExprResult {
        match &self.current.tag {
            TokenTag::Number(x) => {
                let x = *x;
                self.advance()?;
                Ok(Box::new(Expr::Const(x)))
            }
            TokenTag::True => {
                self.advance()?;
                Ok(Box::new(Expr::LiteralBool(true)))
            }
            TokenTag::False => {
                self.advance()?;
                Ok(Box::new(Expr::LiteralBool(false)))
            }
            TokenTag::MinusSign => self.diff(),
            TokenTag::IsZero => self.is_zero(),
            TokenTag::Assert => self.assert(),
            TokenTag::If => self.if_expr(),
            TokenTag::Identifier(var) => {
                let var = var.clone();
                self.advance()?;
                Ok(Box::new(Expr::Var(var)))
            }
            TokenTag::Let => self.let_expr(),
            TokenTag::LetRec => self.let_rec_expr(),
            TokenTag::Proc => self.proc_expr(),
            TokenTag::LeftParen => self.call_expr(),
            unexpected_token => Err(format!("unexpected token `{unexpected_token:}`")),
        }
    }

    fn diff(&mut self) -> ExprResult {
        self.advance()?;
        self.expect(TokenTag::LeftParen)?;
        let left_expr = self.expr()?;
        self.expect(TokenTag::Comma)?;
        let right_expr = self.expr()?;
        self.expect(TokenTag::RightParen)?;

        Ok(Box::new(Expr::Diff(left_expr, right_expr)))
    }

    fn is_zero(&mut self) -> ExprResult {
        self.advance()?;
        self.expect(TokenTag::LeftParen)?;
        let expr = self.expr()?;
        self.expect(TokenTag::RightParen)?;

        Ok(Box::new(Expr::IsZero(expr)))
    }

    fn assert(&mut self) -> ExprResult {
        let line = self.current.line;
        self.advance()?;
        let guard = self.expr()?;
        self.expect(TokenTag::Then)?;
        let body = self.expr()?;

        Ok(Box::new(Expr::Assert { line, guard, body }))
    }

    fn if_expr(&mut self) -> ExprResult {
        self.advance()?;
        let condition = self.expr()?;
        self.expect(TokenTag::Then)?;
        let consequence = self.expr()?;
        self.expect(TokenTag::Else)?;
        let alternative = self.expr()?;

        Ok(Box::new(Expr::If(condition, consequence, alternative)))
    }

    fn let_expr(&mut self) -> ExprResult {
        self.advance()?;
        let var = self.expect_identifer()?;
        self.expect(TokenTag::Equal)?;
        let expr = self.expr()?;
        self.expect(TokenTag::In)?;
        let body = self.expr()?;

        Ok(Box::new(Expr::Let(var, expr, body)))
    }

    fn let_rec_expr(&mut self) -> ExprResult {
        self.advance()?;
        let result_type = self.parse_type()?;
        let name = self.expect_identifer()?;
        self.expect(TokenTag::LeftParen)?;
        let var = self.expect_identifer()?;
        self.expect(TokenTag::Colon)?;
        let var_type = self.parse_type()?;
        self.expect(TokenTag::RightParen)?;
        let proc_body = self.expr()?;
        self.expect(TokenTag::In)?;
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
        self.expect(TokenTag::LeftParen)?;
        let var = self.expect_identifer()?;
        self.expect(TokenTag::Colon)?;
        let ty = self.parse_type()?;
        self.expect(TokenTag::RightParen)?;
        let body = self.expr()?;

        Ok(Box::new(Expr::Proc(var, ty, body)))
    }

    fn call_expr(&mut self) -> ExprResult {
        self.advance()?;
        let operator = self.expr()?;
        let operand = self.expr()?;
        self.expect(TokenTag::RightParen)?;

        Ok(Box::new(Expr::Call(operator, operand)))
    }

    fn parse_type(&mut self) -> Result<LetType, String> {
        match self.current.tag {
            TokenTag::Int => {
                self.advance()?;
                Ok(LetType::new_int())
            }
            TokenTag::Bool => {
                self.advance()?;
                Ok(LetType::new_bool())
            }
            TokenTag::LeftParen => {
                self.advance()?;
                let var_type = self.parse_type()?;
                self.expect(TokenTag::Arrow)?;
                let result_type = self.parse_type()?;
                self.expect(TokenTag::RightParen)?;
                Ok(LetType::new_proc(var_type, result_type))
            }
            _ => Err(format!("unexpected token `{}`", self.current.tag)),
        }
    }
}

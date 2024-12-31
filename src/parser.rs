use crate::expr::{Expr, Expr::*, LiteralValue};
use crate::lexer::{Token, TokenType, TokenType::*};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == Eof
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_token(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison()?;

            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(rhs),
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_token(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let rhs = self.term()?;

            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(rhs),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token(&[Plus, Minus]) {
            let operator = self.previous();
            let rhs = self.factor()?;

            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(rhs),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_token(&[Star, Slash]) {
            let operator = self.previous();
            let rhs = self.unary()?;

            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(rhs),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[Bang, Minus]) {
            let operator = self.previous();
            let rhs = self.unary()?;

            return Ok(Unary {
                operator,
                right: Box::new(rhs),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[LeftParent]) {
            let expr = self.expression()?;
            self.consume(RightParent, "Expected \")\" here")?;
            Ok(Grouping {
                expression: Box::new(expr),
            })
        } else if self.match_token(&[False, True, StringLiteral, Number, Nil]) {
            Ok(Literal {
                value: LiteralValue::from_token(self.previous())?,
            })
        } else {
            Err(format!(
                "Expected expression on line {}",
                self.peek().line_number
            ))
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(msg.to_string())
        }
    }

    fn synhronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }
            match self.previous().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            };

            self.advance();
        }
    }
}

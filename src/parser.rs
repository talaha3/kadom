use crate::expr::{Expr, Expr::*, LiteralValue};
use crate::lexer::{Token, TokenType, TokenType::*};
use crate::stmt::Stmt;

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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements: Vec<Stmt> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(msg) => {
                    self.synchronise();
                    errors.push(msg);
                }
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors.join("\n"))
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(&[Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(Identifier, "Expected variable name")?;

        let mut initialiser = Expr::Literal {
            value: LiteralValue::Nil,
        };

        if self.match_token(&[Equal]) {
            initialiser = self.expression()?;
        }

        self.consume(Semicolon, "Expected \';\' after statement")?;
        Ok(Stmt::Var { name, initialiser })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&[Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let expression = self.expression()?;
        self.consume(Semicolon, "Expected \';\' after statement")?;
        Ok(Stmt::Print { expression })
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expression = self.expression()?;
        println!("{expression}");
        self.consume(Semicolon, "Expected \';\' after statement")?;
        Ok(Stmt::Expression { expression })
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
            self.consume(RightParent, "Expected \')\' here")?;
            Ok(Grouping {
                expression: Box::new(expr),
            })
        } else if self.match_token(&[Identifier]) {
            Ok(Variable {
                name: self.previous(),
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

    fn synchronise(&mut self) {
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

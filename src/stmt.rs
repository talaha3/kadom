use crate::{
    expr::Expr,
    lexer::{Token, TokenType},
};

pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initialiser: Expr },
}

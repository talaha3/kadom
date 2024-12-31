use crate::{
    environment::Environment,
    expr::{Expr, LiteralValue},
    stmt::Stmt,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Stmt::Print { expression } => {
                    let value = expression.evaluate(&self.environment)?;
                    println!("{}", value)
                }
                Stmt::Expression { expression } => {
                    expression.evaluate(&self.environment)?;
                }
                Stmt::Var { name, initialiser } => {
                    let value = initialiser.evaluate(&self.environment)?;

                    self.environment.define(name.lexeme, value);
                }
            }
        }

        Ok(())
    }
}

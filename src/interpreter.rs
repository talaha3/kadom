use crate::expr::{Expr, LiteralValue};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expression: Expr) -> Result<LiteralValue, String> {
        expression.evaluate()
    }
}

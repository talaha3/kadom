use crate::lexer::{Token, TokenType};

pub enum LiteralValue {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string_value = match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.clone(),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
            Self::Nil => "nil".to_string(),
        };

        write!(f, "{}", string_value)
    }
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator.lexeme, left, right),
            Self::Grouping { expression } => write!(f, "(group {})", expression),
            Self::Literal { value } => write!(f, "{}", value),
            Self::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
        }
    }
}

impl Expr {
    fn print(&self) {
        println!("{}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::*;
    use super::LiteralValue::*;
    use super::*;

    #[test]
    fn printy_print_ast() {
        let minus_token = Token::new(TokenType::Minus, "-".to_string(), None, u64::MAX);
        let one_two_three = Literal {
            value: Number(123.0),
        };
        let group = Grouping {
            expression: Box::new(Literal {
                value: Number(45.67),
            }),
        };
        let multiply_token = Token::new(TokenType::Star, "*".to_string(), None, u64::MAX);
        let ast = Binary {
            left: Box::new(Unary {
                operator: minus_token,
                right: Box::new(one_two_three),
            }),
            operator: multiply_token,
            right: Box::new(group),
        };

        assert_eq!(ast.to_string(), "(* (- 123) (group 45.67))");
    }
}

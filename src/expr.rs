use crate::{
    environment::Environment,
    lexer::{self, Token, TokenType},
};

fn unwrap_as_f32(literal: Option<lexer::LiteralValue>) -> Result<f32, String> {
    match literal {
        Some(lexer::LiteralValue::IntVal(s)) => Ok(s as f32),
        Some(lexer::LiteralValue::FVal(s)) => Ok(s as f32),
        _ => Err("Could not unwrap as f32".to_string()),
    }
}

fn unwrap_as_string(literal: Option<lexer::LiteralValue>) -> Result<String, String> {
    match literal {
        Some(lexer::LiteralValue::StringVal(s)) => Ok(s),
        Some(lexer::LiteralValue::IdentifierVal(s)) => Ok(s),
        _ => Err("Could not unwrap as string".to_string()),
    }
}

#[derive(Debug, Clone, PartialEq)]
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

impl LiteralValue {
    pub fn from_token(token: Token) -> Result<Self, String> {
        match token.token_type {
            TokenType::Number => Ok(Self::Number(unwrap_as_f32(token.literal_option)?)),
            TokenType::StringLiteral => Ok(Self::String(unwrap_as_string(token.literal_option)?)),
            TokenType::False => Ok(Self::False),
            TokenType::True => Ok(Self::True),
            TokenType::Nil => Ok(Self::Nil),
            _ => panic!("Could not convert to LiteralValue"),
        }
    }

    fn not(&self) -> Self {
        match self {
            Self::False | Self::Nil => Self::True,
            Self::Number(x) => {
                if *x == 0 as f32 {
                    Self::True
                } else {
                    Self::False
                }
            }
            Self::String(str) => {
                if str.is_empty() {
                    Self::True
                } else {
                    Self::False
                }
            }
            Self::True => Self::False,
        }
    }

    fn from_bool(boolean: bool) -> Self {
        match boolean {
            true => Self::True,
            false => Self::False,
        }
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
    Variable {
        name: Token,
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
            Self::Variable { name } => {
                write!(f, "var {}", name.lexeme)
            }
        }
    }
}

impl Expr {
    pub fn evaluate(&self, environment: &Environment) -> Result<LiteralValue, String> {
        match self {
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping { expression } => Ok(expression.evaluate(environment)?),
            Expr::Unary { operator, right } => {
                let evaluate_right = right.evaluate(environment)?;

                match (evaluate_right, &operator.token_type) {
                    (LiteralValue::Number(x), TokenType::Minus) => Ok(LiteralValue::Number(-x)),
                    (non_number, TokenType::Minus) => {
                        Err(format!("Negation not implemented for {:?}", non_number))
                    }
                    (any, TokenType::Bang) => Ok(any.not()),
                    (_, _) => Err("Unreachable".to_string()),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let evaluate_left = left.evaluate(environment)?;
                let evaluate_right = right.evaluate(environment)?;

                match (evaluate_left, &operator.token_type, evaluate_right) {
                    (LiteralValue::Number(x), TokenType::Minus, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x - y))
                    }
                    (LiteralValue::Number(x), TokenType::Slash, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x / y))
                    }
                    (LiteralValue::Number(x), TokenType::Star, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x * y))
                    }
                    (LiteralValue::Number(x), TokenType::Plus, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x + y))
                    }
                    (LiteralValue::String(str1), TokenType::Plus, LiteralValue::String(str2)) => {
                        Ok(LiteralValue::String(str1 + str2.as_str()))
                    }
                    (LiteralValue::Number(x), TokenType::Greater, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }
                    (LiteralValue::Number(x), TokenType::GreaterEqual, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x >= y))
                    }
                    (LiteralValue::Number(x), TokenType::Less, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x < y))
                    }
                    (LiteralValue::Number(x), TokenType::LessEqual, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x <= y))
                    }
                    (LiteralValue::String(x), TokenType::Greater, LiteralValue::String(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }
                    (LiteralValue::String(x), TokenType::GreaterEqual, LiteralValue::String(y)) => {
                        Ok(LiteralValue::from_bool(x >= y))
                    }
                    (LiteralValue::String(x), TokenType::Less, LiteralValue::String(y)) => {
                        Ok(LiteralValue::from_bool(x < y))
                    }
                    (LiteralValue::String(x), TokenType::LessEqual, LiteralValue::String(y)) => {
                        Ok(LiteralValue::from_bool(x <= y))
                    }
                    (x, TokenType::EqualEqual, y) => Ok(LiteralValue::from_bool(x == y)),
                    (x, TokenType::BangEqual, y) => Ok(LiteralValue::from_bool(x != y)),

                    // Error handling
                    (LiteralValue::String(_), oper, LiteralValue::Number(_)) => {
                        Err(format!("Mismatched types for {oper:?}: String and Number"))
                    }
                    (LiteralValue::Number(_), oper, LiteralValue::String(_)) => {
                        Err(format!("Mismatched types for {oper:?}: Number and String"))
                    }
                    (x, oper, y) => Err(format!(
                        "{:?} cannot be evaluated for {:?} and {:?}",
                        oper, x, y
                    )),
                }
            }
            Self::Variable { name } => environment.get(&name.lexeme),
        }
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

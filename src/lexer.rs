use std::fmt::{self, Write};
use LiteralValue::*;
use TokenType::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors = Vec::new();

        while self.is_at_end() {
            self.start = self.current;
            let _ = self.scan_token().map_err(|e| errors.push(e));
        }

        self.tokens
            .push(Token::new(Eof, "".into(), None, self.line));

        if !errors.is_empty() {
            let mut joined = "".to_string();
            errors.iter().for_each(|msg| {
                joined.push_str(msg);
                joined.push('\n');
            });

            return Err(joined);
        }

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token_null_literal(LeftParent),
            ')' => self.add_token_null_literal(RightParent),
            '{' => self.add_token_null_literal(LeftBrace),
            '}' => self.add_token_null_literal(LeftBrace),
            ',' => self.add_token_null_literal(Comma),
            '.' => self.add_token_null_literal(Dot),
            '-' => self.add_token_null_literal(Minus),
            '+' => self.add_token_null_literal(Plus),
            ';' => self.add_token_null_literal(Semicolon),
            '*' => self.add_token_null_literal(Star),

            // Operators
            '=' => {
                let token = if self.match_char('=') {
                    EqualEqual
                } else {
                    Equal
                };

                self.add_token_null_literal(token)
            }
            '!' => {
                let token = if self.match_char('=') {
                    BangEqual
                } else {
                    Bang
                };

                self.add_token_null_literal(token)
            }
            '<' => {
                let token = if self.match_char('=') {
                    LessEqual
                } else {
                    Less
                };

                self.add_token_null_literal(token)
            }
            '>' => {
                let token = if self.match_char('=') {
                    GreaterEqual
                } else {
                    Greater
                };

                self.add_token_null_literal(token)
            }
            // Comments or Division
            '/' => {
                if self.match_char('/') {
                    loop {
                        if self.peek() == '\n' || !self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                }
                self.add_token_null_literal(Slash)
            }

            // Whitespace
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            }

            // Literals
            '"' => self.string_literal(),
            _ => Err(format!(
                "Oopsie, character not recognised: {} at line {}",
                c, self.line
            )),
        }
    }

    fn source_char_at_current(&self) -> char {
        let c = self.source.as_bytes()[self.current];
        c as char
    }

    fn advance(&mut self) -> char {
        let c = self.source_char_at_current();
        self.current += 1;
        c
    }

    fn match_char(&mut self, character: char) -> bool {
        if self.is_at_end() || self.source_char_at_current() != character {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn add_token_null_literal(&mut self, token_type: TokenType) -> Result<(), String> {
        self.add_token(token_type, None);
        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source_char_at_current()
        }
    }

    fn string_literal(&mut self) -> Result<(), String> {
        loop {
            if self.peek() == '"' || self.is_at_end() {
                break;
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(format!("unterminated string lol :/ on line {}", self.line));
        }

        self.advance();
        let value_as_str = &self.source[self.start + 1..self.current - 1];
        let value = StringVal(value_as_str.into());
        self.add_token(StringLiteral, Some(value));
        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType, literal_option: Option<LiteralValue>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            text.into(),
            literal_option,
            self.line,
        ));
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParent,
    RightParent,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringLiteral,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntVal(i64),
    FVal(f64),
    StringVal(String),
    IdentifierVal(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal_option: Option<LiteralValue>,
    line_number: u64,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal_option: Option<LiteralValue>,
        line_number: u64,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal_option,
            line_number,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal_option
        )
    }
}

use std::collections::HashMap;
use std::fmt::{self};
use LiteralValue::*;
use TokenType::*;

fn is_alpha(c: char) -> bool {
    (c as u8 >= b'a' && c as u8 <= b'z') || (c as u8 >= b'A' && c as u8 <= b'Z') || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || c.is_ascii_digit()
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert("and".to_string(), And);
        keywords.insert("class".to_string(), Class);
        keywords.insert("else".to_string(), Else);
        keywords.insert("false".to_string(), False);
        keywords.insert("for".to_string(), For);
        keywords.insert("fun".to_string(), Fun);
        keywords.insert("if".to_string(), If);
        keywords.insert("nil".to_string(), Nil);
        keywords.insert("or".to_string(), Or);
        keywords.insert("print".to_string(), Print);
        keywords.insert("return".to_string(), Return);
        keywords.insert("super".to_string(), Super);
        keywords.insert("this".to_string(), This);
        keywords.insert("true".to_string(), True);
        keywords.insert("var".to_string(), Var);
        keywords.insert("while".to_string(), While);

        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors = Vec::new();

        while !self.is_at_end() {
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
            '}' => self.add_token_null_literal(RightBrace),
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

            // String Literals
            '"' => self.string_literal(),

            c => {
                // Number Literals
                if c.is_ascii_digit() {
                    self.number()
                } else if is_alpha(c) {
                    self.identifier()
                } else {
                    Err(format!(
                        "Oopsie, character not recognised: {} at line {}",
                        c, self.line
                    ))
                }
            }
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
        self.add_token(token_type, None)?;
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
        self.add_token(StringLiteral, Some(value))?;
        Ok(())
    }

    fn number(&mut self) -> Result<(), String> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let digit_value = self.source[self.start..self.current]
            .parse::<f64>()
            .map_err(|_| "Could not parse as f64")?;
        self.add_token(Number, Some(FVal(digit_value)))?;
        Ok(())
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            let c = self.source.as_bytes()[self.current + 1];
            c as char
        }
    }

    fn identifier(&mut self) -> Result<(), String> {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let token_type = self.keywords.get(&text).unwrap_or(&Identifier).clone();
        self.add_token_null_literal(token_type)
    }

    fn add_token(
        &mut self,
        token_type: TokenType,
        literal_option: Option<LiteralValue>,
    ) -> Result<(), String> {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            text.into(),
            literal_option,
            self.line,
        ));
        Ok(())
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

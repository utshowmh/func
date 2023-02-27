use std::collections::HashMap;

use crate::common::{
    error::{Error, ErrorType},
    object::Object,
    position::Position,
    token::{Token, TokenType},
};

pub struct Lexer {
    source: Vec<char>,

    keywords: HashMap<String, TokenType>,

    start: usize,
    current: usize,

    current_position: Position,
}

impl Lexer {
    pub fn new(source_path: String, source: &str) -> Self {
        Self {
            source: source.chars().collect(),

            keywords: HashMap::new(),

            start: 0,
            current: 0,

            current_position: Position::new(source_path, 1),
        }
    }

    fn init_keywords(&mut self) {
        self.keywords.insert("let".to_string(), TokenType::Let);
        self.keywords.insert("func".to_string(), TokenType::Func);
        self.keywords.insert("read".to_string(), TokenType::Read);
        self.keywords.insert("write".to_string(), TokenType::Write);
        self.keywords.insert("if".to_string(), TokenType::If);
        self.keywords.insert("else".to_string(), TokenType::Else);
        self.keywords.insert("true".to_string(), TokenType::Boolean);
        self.keywords
            .insert("false".to_string(), TokenType::Boolean);
        self.keywords.insert("nil".to_string(), TokenType::Nil);
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        self.init_keywords();

        let mut tokens = Vec::new();

        while !self.eof() {
            self.start = self.current;
            if let Some(token) = self.next_token()? {
                tokens.push(token);
            }
        }

        tokens.push(self.token(TokenType::EOF, None));
        Ok(tokens)
    }

    fn eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if !self.eof() {
            self.source[self.current]
        } else {
            // We're never reaching this. This is just to trick the compiler.
            '\0'
        }
    }

    fn advance(&mut self) {
        if !self.eof() {
            self.current += 1;
        }
    }

    fn token(&mut self, ttype: TokenType, literal: Option<Object>) -> Token {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        Token::new(ttype, lexeme, literal, self.current_position.clone())
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        let current_char = self.peek();
        self.advance();
        match current_char {
            ' ' | '\t' | '\r' => Ok(None),

            '+' => Ok(Some(self.token(TokenType::Plus, None))),

            '-' => Ok(Some(self.token(TokenType::Minus, None))),

            '*' => Ok(Some(self.token(TokenType::Star, None))),

            '%' => Ok(Some(self.token(TokenType::Modulo, None))),

            ',' => Ok(Some(self.token(TokenType::Comma, None))),

            '(' => Ok(Some(self.token(TokenType::OpenParen, None))),

            ')' => Ok(Some(self.token(TokenType::CloseParen, None))),

            '[' => Ok(Some(self.token(TokenType::OpenBrack, None))),

            ']' => Ok(Some(self.token(TokenType::CloseBrack, None))),

            '{' => Ok(Some(self.token(TokenType::OpenCurly, None))),

            '}' => Ok(Some(self.token(TokenType::CloseCurly, None))),

            '\0' => Ok(Some(self.token(TokenType::EOF, None))),

            '"' => self.make_string(),

            '\n' => self.count_newline(),

            '/' => {
                if self.peek() == '/' {
                    self.advance();
                    self.ignore_comment()
                } else {
                    Ok(Some(self.token(TokenType::Slash, None)))
                }
            }

            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Some(self.token(TokenType::EqualEqual, None)))
                } else {
                    Ok(Some(self.token(TokenType::Equal, None)))
                }
            }

            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Some(self.token(TokenType::NotEqual, None)))
                } else {
                    Ok(Some(self.token(TokenType::Not, None)))
                }
            }

            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Some(self.token(TokenType::GreaterEqual, None)))
                } else {
                    Ok(Some(self.token(TokenType::Greater, None)))
                }
            }

            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Some(self.token(TokenType::LessEqual, None)))
                } else {
                    Ok(Some(self.token(TokenType::Less, None)))
                }
            }

            '&' => {
                if self.peek() == '&' {
                    self.advance();
                    Ok(Some(self.token(TokenType::And, None)))
                } else {
                    Err(Error::new(
                        ErrorType::LexingError,
                        format!("Unexpected charected `{}`", current_char),
                        self.current_position.clone(),
                    ))
                }
            }

            '|' => {
                if self.peek() == '|' {
                    self.advance();
                    Ok(Some(self.token(TokenType::Or, None)))
                } else {
                    Err(Error::new(
                        ErrorType::LexingError,
                        format!("Unexpected charected `{}`", current_char),
                        self.current_position.clone(),
                    ))
                }
            }

            _ => {
                if current_char.is_ascii_alphabetic() || current_char == '_' {
                    self.make_identifier()
                } else if current_char.is_ascii_digit() {
                    self.make_number()
                } else {
                    Err(Error::new(
                        ErrorType::LexingError,
                        format!("Unexpected charected `{}`", current_char),
                        self.current_position.clone(),
                    ))
                }
            }
        }
    }

    fn count_newline(&mut self) -> Result<Option<Token>, Error> {
        self.current_position.row += 1;
        Ok(None)
    }

    fn ignore_comment(&mut self) -> Result<Option<Token>, Error> {
        while self.peek() != '\n' && !self.eof() {
            self.advance();
        }
        Ok(None)
    }

    fn make_string(&mut self) -> Result<Option<Token>, Error> {
        loop {
            if self.peek() == '"' || self.peek() == '\0' {
                break;
            }
            self.advance();
        }
        if self.peek() == '"' {
            self.advance();
            let lexeme: String = self.source[self.start + 1..self.current - 1]
                .iter()
                .collect();
            Ok(Some(
                self.token(TokenType::String, Some(Object::String(lexeme))),
            ))
        } else {
            Err(Error::new(
                ErrorType::LexingError,
                format!("Unterminated string"),
                self.current_position.clone(),
            ))
        }
    }

    fn make_number(&mut self) -> Result<Option<Token>, Error> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        if let Ok(float) = lexeme.parse() {
            Ok(Some(
                self.token(TokenType::Number, Some(Object::Number(float))),
            ))
        } else {
            Err(Error::new(
                ErrorType::LexingError,
                format!("could not parse {} to float", lexeme),
                self.current_position.clone(),
            ))
        }
    }

    fn make_identifier(&mut self) -> Result<Option<Token>, Error> {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        if let Some(ttype) = self.keywords.get(&lexeme) {
            if ttype == &TokenType::Boolean && lexeme == "true" {
                Ok(Some(
                    self.token(TokenType::Boolean, Some(Object::Boolean(true))),
                ))
            } else if ttype == &TokenType::Boolean && lexeme == "false" {
                Ok(Some(
                    self.token(TokenType::Boolean, Some(Object::Boolean(false))),
                ))
            } else {
                Ok(Some(self.token(ttype.clone(), None)))
            }
        } else {
            Ok(Some(self.token(TokenType::Identifier, None)))
        }
    }
}

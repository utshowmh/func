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

            current_position: Position::new(source_path, 0, 1),
        }
    }

    fn init_keywords(&mut self) {
        self.keywords.insert("let".to_string(), TokenType::Let);
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
            self.current_position.column += 1;
        }
    }

    fn token(&self, ttype: TokenType, literal: Option<Object>) -> Token {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        Token::new(ttype, lexeme, literal, self.current_position.clone())
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        let current_char = self.peek();
        self.advance();
        match current_char {
            ' ' | '\t' | '\r' => Ok(None),

            '=' => Ok(Some(self.token(TokenType::Equal, None))),
            '+' => Ok(Some(self.token(TokenType::Plus, None))),
            '-' => Ok(Some(self.token(TokenType::Minus, None))),
            '*' => Ok(Some(self.token(TokenType::Star, None))),
            '/' => Ok(Some(self.token(TokenType::Slash, None))),
            '%' => Ok(Some(self.token(TokenType::Modulo, None))),
            '(' => Ok(Some(self.token(TokenType::OpenParen, None))),
            ')' => Ok(Some(self.token(TokenType::CloseParen, None))),
            '\0' => Ok(Some(self.token(TokenType::EOF, None))),

            '#' => {
                while self.peek() != '\n' && !self.eof() {
                    self.advance();
                }
                Ok(None)
            }

            '\n' => {
                self.current_position.row += 1;
                self.current_position.column = 0;
                Ok(None)
            }

            _ => {
                if current_char.is_ascii_alphabetic() {
                    while self.peek().is_ascii_alphanumeric() {
                        self.advance();
                    }
                    let lexeme: String = self.source[self.start..self.current].iter().collect();
                    if let Some(ttype) = self.keywords.get(&lexeme) {
                        Ok(Some(self.token(ttype.clone(), None)))
                    } else {
                        Ok(Some(self.token(TokenType::Identifier, None)))
                    }
                } else if current_char.is_ascii_digit() {
                    while self.peek().is_ascii_digit() {
                        self.advance();
                    }
                    if self.peek() == '.' {
                        self.advance();
                        while self.peek().is_ascii_digit() {
                            self.advance();
                        }
                        let lexeme: String = self.source[self.start..self.current].iter().collect();
                        if let Ok(float) = lexeme.parse() {
                            Ok(Some(
                                self.token(TokenType::Float, Some(Object::Float(float))),
                            ))
                        } else {
                            panic!("could not parse {} to float", lexeme);
                        }
                    } else {
                        let lexeme: String = self.source[self.start..self.current].iter().collect();
                        if let Ok(int) = lexeme.parse() {
                            Ok(Some(
                                self.token(TokenType::Integer, Some(Object::Integer(int))),
                            ))
                        } else {
                            panic!("could not parse {} to int", lexeme);
                        }
                    }
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
}

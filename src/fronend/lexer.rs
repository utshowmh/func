use std::collections::HashMap;

use crate::common::{
    error::{Error, ErrorType},
    object::Object,
    token::{Token, TokenType},
};

pub struct Lexer {
    source: Vec<char>,

    keywords: HashMap<String, TokenType>,

    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),

            keywords: HashMap::new(),

            start: 0,
            current: 0,
            line: 1,
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

        Ok(tokens)
    }

    fn eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if !self.eof() {
            self.source[self.current]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) {
        if !self.eof() {
            self.current += 1;
        }
    }

    fn token(&self, ttype: TokenType, literal: Option<Object>) -> Token {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        Token::new(ttype, lexeme, literal, self.line)
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        let current_char = self.peek();
        self.advance();
        match current_char {
            '=' => Ok(Some(self.token(TokenType::Equal, None))),
            '+' => Ok(Some(self.token(TokenType::Plus, None))),
            '-' => Ok(Some(self.token(TokenType::Minus, None))),
            '*' => Ok(Some(self.token(TokenType::Star, None))),
            '/' => Ok(Some(self.token(TokenType::Slash, None))),
            '%' => Ok(Some(self.token(TokenType::Modulo, None))),
            '(' => Ok(Some(self.token(TokenType::OpenParen, None))),
            ')' => Ok(Some(self.token(TokenType::CloseParen, None))),

            _ => {
                if current_char == ' ' || current_char == '\t' || current_char == '\r' {
                    Ok(None)
                } else if current_char == '\n' {
                    self.line += 1;
                    Ok(None)
                } else if current_char.is_ascii_alphabetic() {
                    while self.peek().is_ascii_alphabetic() {
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
                    let lexeme: String = self.source[self.start..self.current].iter().collect();
                    if let Ok(int) = lexeme.parse() {
                        Ok(Some(
                            self.token(TokenType::Integer, Some(Object::Integer(int))),
                        ))
                    } else {
                        todo!()
                    }
                } else {
                    Err(Error::new(
                        ErrorType::LexingError,
                        format!("Unexpected charected `{}`", current_char),
                        self.line,
                    ))
                }
            }
        }
    }
}

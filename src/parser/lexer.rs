use super::token;
use crate::parser::token::{Token, TokenType};

use std::fs;

pub struct Lexer {
    current: char,
    ptr: usize,
    content: Vec<u8>,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(filepath: &str) -> Self {
        let mut lexer = Lexer {
            current: '\0',
            ptr: 0,
            content: fs::read(filepath).expect("[ERROR] Failed to read file."),
            tokens: vec![],
        };

        lexer.current = lexer.content[0] as char;

        return lexer;
    }

    fn advance(&mut self) {
        if self.ptr < self.content.len() - 1 {
            self.ptr += 1;
            self.current = char::from(self.content[self.ptr]);
        } else {
            self.current = '\0'
        }
    }

    fn push_token(&mut self, t: TokenType, v: &str) {
        self.tokens.push(Token::new(t, v));
    }

    fn skip_whitespace(&mut self) {
        while (self.current == ' ' || self.current == '\t') && self.ptr != self.content.len() {
            self.advance();
        }
    }

    fn get_string(&mut self) -> Token {
        self.advance();

        let mut string: String = String::new();

        while self.current != '"' && self.ptr != self.content.len() {
            string.push(self.current);
            self.advance()
        }

        self.advance();

        return Token::new(TokenType::TokenString, string.as_str());
    }

    fn get_id(&mut self) -> Token {
        let mut string: String = String::new();

        while (self.current != ' ' && self.current != '\t' && self.current != '\0')
            && self.ptr < self.content.len()
        {
            if Token::is_single_token(self.current) {
                return Token::new(TokenType::TokenID, string.as_str());
            }

            string.push(self.current);
            self.advance();
        }

        return Token::new(TokenType::TokenID, string.as_str());
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        // Collect tokens
        loop {
            // Advance
            if self.current == '\0' && self.ptr < self.content.len() {
                break;
            }

            // Skip whitespace
            self.skip_whitespace();

            // Collect string
            if self.current == '"' {
                return Some(self.get_string());
            }

            // Collect id
            if self.current.is_alphanumeric() || self.current == '-' || self.current == '+' {
                return Some(self.get_id());
            }

            // Collect token
            match self.current {
                '=' => {
                    self.advance();
                    return Some(Token::new(TokenType::TokenEqual, "="));
                }
                '[' => {
                    self.advance();
                    return Some(Token::new(TokenType::TokenLBracket, "["));
                }
                ']' => {
                    self.advance();
                    return Some(Token::new(TokenType::TokenRBracket, "]"));
                }
                ',' => {
                    self.advance();
                    return Some(Token::new(TokenType::TokenComma, ","));
                }
                '\n' => {
                    self.advance();
                    return Some(Token::new(TokenType::TokenNewLine, "\n"));
                }
                _ => {}
            };
        }

        return None;
    }
}

use crate::parser::ast::AST;
use crate::parser::lexer::Lexer;
use crate::parser::token::{Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    token: Option<Token>,
}

impl Parser {
    pub fn new(filepath: &str) -> Self {
        return Parser {
            lexer: Lexer::new(filepath),
            token: None,
        };
    }

    pub fn get_token(&self) -> Token {
        return if let Some(t) = &self.token {
            Token::new(t.token_type.clone(), t.value.as_str())
        } else {
            Token::new(TokenType::TokenNone, "\0")
        };
    }

    pub fn consume(&mut self, token_type: TokenType) {
        if let Some(t) = &self.token {
            if t.token_type == token_type {
                self.token = self.lexer.get_next_token();
            } else {
                panic!(
                    "[ERROR] Unexpected token {:?}, with value '{}'.",
                    t.token_type, t.value
                );
            }
        } else {
            println!("[WARNING] Token is None.");
        }
    }

    pub fn parse(&mut self) -> AST {
        self.token = self.lexer.get_next_token();

        return self.parse_multiple_statement();
    }

    pub fn parse_statement(&mut self) -> AST {
        if let Some(token) = &mut self.token {
            match token.token_type {
                TokenType::TokenNone => {}
                TokenType::TokenID => {
                    return self.parse_variable();
                }
                TokenType::TokenString => {}
                TokenType::TokenEqual => {}
                TokenType::TokenComma => {}
                TokenType::TokenNewLine => {
                    return AST::ASTSeparator();
                }
                TokenType::TokenLBracket => {
                    return self.parse_tag();
                }
                TokenType::TokenRBracket => {}
            }

            panic!(
                "[ERROR] Unexpected token {:?}, with value '{}'.",
                token.token_type, token.value
            );
        } else {
            panic!("[ERROR] Token is None.");
        }
    }

    pub fn parse_multiple_statement(&mut self) -> AST {
        let mut compound = AST::ASTCompound(vec![]);

        let statement = self.parse_statement();

        let mut token = self.get_token();

        if let AST::ASTCompound(v) = &mut compound {
            v.push(statement);
        }

        while token.token_type == TokenType::TokenNewLine {
            if let Some(t) = &mut self.token {
                token.token_type = t.token_type.clone();
                token.value = t.value.clone();

                self.consume(TokenType::TokenNewLine);
            } else {
                break;
            }

            let ast_statement = self.parse_statement();

            if let AST::ASTCompound(ast) = &mut compound {
                ast.push(ast_statement);
            } else {
                panic!("[ERROR] Compound is of wrong type.");
            }
        }

        return compound;
    }

    pub fn parse_tag(&mut self) -> AST {
        self.consume(TokenType::TokenLBracket); // Consume the left braces

        let tag: String = self.get_token().value; // Save the tag name

        self.consume(TokenType::TokenID);

        self.consume(TokenType::TokenRBracket);

        return AST::ASTTagDefinition(tag);
    }

    pub fn parse_variable(&mut self) -> AST {
        let var_name: String = self.get_token().value;

        self.consume(TokenType::TokenID); // Consume the variable name

        self.consume(TokenType::TokenEqual); // Consume the equals sign

        let ast_value = self.parse_value();

        return AST::ASTVariableDefinition(var_name, Box::new(ast_value));
    }

    pub fn parse_value(&mut self) -> AST {
        let token: Token = self.get_token();
        let var_value: String;

        match &token.token_type {
            TokenType::TokenID => {
                //var_value = token.value;
                return self.parse_value_from_id();
            }
            TokenType::TokenString => {
                return self.parse_string();
            }
            TokenType::TokenLBracket => {
                return self.parse_array();
            }
            _ => {
                panic!(
                    "[ERROR] Unexpected token {:?}, with value '{}'.",
                    token.token_type, token.value
                );
            }
        }
    }

    pub fn parse_value_from_id(&mut self) -> AST {
        let token: Token = self.get_token();

        self.consume(TokenType::TokenID);

        return if token.value == "false" || token.value == "true" {
            if token.value == "true" {
                AST::ASTBool(true)
            } else {
                AST::ASTBool(false)
            }
        } else {
            AST::ASTInt(token.value.parse::<i32>().unwrap())
        };
    }

    pub fn parse_string(&mut self) -> AST {
        let token = self.get_token();
        self.consume(TokenType::TokenString);

        return AST::ASTString(token.value);
    }

    pub fn parse_array(&mut self) -> AST {
        self.consume(TokenType::TokenLBracket); // Consume the bracket

        let mut array: Vec<AST> = vec![];
        let mut expected_value: bool = true;

        while let token = self.get_token() {
            match token.token_type {
                TokenType::TokenID => {
                    if !expected_value {
                        panic!(
                            "[ERROR] Unexpected token {:?}, with value '{}'.",
                            token.token_type, token.value
                        );
                    } else {
                        let val = self.parse_value_from_id();

                        array.push(val);
                        expected_value = false;
                    }
                }
                TokenType::TokenString => {
                    if !expected_value {
                        panic!(
                            "[ERROR] Unexpected token {:?}, with value '{}'.",
                            token.token_type, token.value
                        );
                    } else {
                        let val = self.parse_string();

                        array.push(val);
                        expected_value = false;
                    }
                }
                TokenType::TokenComma => {
                    if expected_value {
                        panic!(
                            "[ERROR] Unexpected token {:?}, with value '{}'.",
                            token.token_type, token.value
                        );
                    }
                    expected_value = true;

                    self.consume(TokenType::TokenComma);
                }
                TokenType::TokenRBracket => {
                    if expected_value {
                        panic!(
                            "[ERROR] Unexpected token {:?}, with value '{}'.",
                            token.token_type, token.value
                        );
                    }

                    self.consume(TokenType::TokenRBracket);
                    break;
                }
                TokenType::TokenNewLine => {
                    self.consume(TokenType::TokenNewLine);
                }
                TokenType::TokenLBracket => {
                    if !expected_value {
                        panic!(
                            "[ERROR] Unexpected token {:?}, with value '{}'.",
                            token.token_type, token.value
                        );
                    }

                    array.push(self.parse_array());

                    expected_value = false;
                }
                _ => {
                    panic!(
                        "[ERROR] Unexpected token {:?}, with value '{}'.",
                        token.token_type, token.value
                    );
                }
            }
        }

        return AST::ASTArray(array);
    }
}

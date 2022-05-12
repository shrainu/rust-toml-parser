
#[derive(Debug)]
pub enum TokenType {
    TokenID,
    TokenString,
    TokenEqual,
    TokenComma,
    TokenLBracket,
    TokenRBracket,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String
}

impl Token {
    pub fn new(t: TokenType, v: &str) -> Self {
        return Token {
            token_type: t,
            value: String::from(v)
        }
    }

    pub fn is_single_token(s: char) -> bool {
        return match s {
            '=' | '[' | ']' | ',' => { true },
            _ => { false }
        }
    }
}


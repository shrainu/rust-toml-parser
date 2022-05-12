use crate::parser::token::Token;

mod parser;

fn main() {

    let mut lexer = parser::lexer::Lexer::new("product/test.toml");

    loop {
        let token = lexer.get_next_token();
        match token {
            None => { break; }
            Some(t) => { println!("Token {{ {:?}, '{}' }}", t.token_type, t.value); }
        }
    }
}

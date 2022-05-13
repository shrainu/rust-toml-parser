extern crate core;

use crate::parser::ast::AST;
use crate::parser::converter::convert_ast_to_string;
use crate::parser::parser::Parser;
use crate::parser::token::Token;

mod parser;

fn main() {
    let mut lexer = parser::lexer::Lexer::new("product/test.toml");

    loop {
        let token = lexer.get_next_token();
        match token {
            None => {
                break;
            }
            Some(t) => {
                println!(
                    "Token {{ {:?}, '{}' }}",
                    t.token_type,
                    if t.value == "\n" {
                        "\\n"
                    } else {
                        t.value.as_str()
                    }
                );
            }
        }
    }

    println!("[INFO] PARSING.\n");

    let mut pars: Parser = Parser::new("product/test.toml");

    let ast: AST = pars.parse();

    if let AST::ASTCompound(vec) = &ast {
        for a in vec {
            println!("AST: {:?}", a);
        }
    }

    println!();

    let map = convert_ast_to_string(&ast, false);

    for (name, tag) in map.tags.iter() {
        println!("[{}]", name.to_uppercase());
        for (k, v) in tag.values.iter() {
            if v.contains(';') {
                println!("{{ K: {}, V: [{}] }}", k, v);
            } else {
                println!("{{ K: {}, V: {} }}", k, v);
            }
        }
        println!();
    }
}

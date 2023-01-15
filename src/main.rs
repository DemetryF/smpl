use std::fs;

use lexer::Lexer;

use crate::{
    lexer::token::token_value::TokenValue,
    parser::{token_stream::TokenStream, Parser},
};

mod lexer;
mod parser;

fn main() {
    if let Ok(code) = fs::read_to_string("test.smpl") {
        println!("{}", code);

        let mut p = Parser::new(&code);

        p.statement();
    }
}

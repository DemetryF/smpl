use std::fs;

use crate::parser::Parser;

mod lexer;
mod parser;

fn main() {
    if let Ok(code) = fs::read_to_string("test.smpl") {
        println!("{}", code);

        let mut p = Parser::new(&code);

        p.statement();
    }
}

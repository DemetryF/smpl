use std::fs;

use crate::parser::Parser;

mod lexer;
mod parser;
mod translator;

fn main() {
    if let Ok(code) = fs::read_to_string("test.smpl") {
        println!("{}", code);

        let mut p = Parser::new(&code);

        println!("{:?}", p.parse());
    }
}

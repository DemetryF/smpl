use std::fs;

use crate::translator::Translator;

mod error;
mod lexer;
mod parser;
mod translator;

fn main() {
    if let Ok(code) = fs::read_to_string("test.smpl") {
        println!("{}", code);

        let mut t = Translator::new(code);

        t.translate();

        for i in t.instructions {
            println!("{}", i);
        }
    }
}

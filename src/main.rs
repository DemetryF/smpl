use std::{env, fs};

use crate::translator::Translator;

mod error;
mod lexer;
mod parser;
mod static_analyzer;
mod translator;

fn main() {
    let filename = env::args().last().expect("no input file");

    if let Ok(code) = fs::read_to_string(filename.clone()) {
        println!("{}", code);

        let mut t = Translator::new(code);

        t.translate();

        for i in t.instructions {
            println!("{}", i);
        }
    } else {
        println!("no such file \"{}\"", filename);
    }
}

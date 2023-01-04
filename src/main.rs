use std::fs;

use lexer::Lexer;

mod lexer;

fn main() {
    if let Ok(code) = fs::read_to_string("test.smpl") {
        println!("{}", code);
        let mut l: Lexer = Lexer::new(code);

        while let Some(token) = l.next_token() {
            match token {
                Ok(t) => println!("{:?}", t),
                Err(err) => println!("{:?}", err),
            }
        }
    }
}

use std::fs;

use lexer::Lexer;

use crate::lexer::token::token_value::TokenValue;

mod lexer;

fn main() {
    if let Ok(code) = fs::read_to_string("test.smpl") {
        println!("{}", code);
        let mut l: Lexer = Lexer::new(&code);

        loop {
            match l.next_token() {
                Ok(t) => {
                    println!("{:?}", t);
                    if let TokenValue::Eof = t.value {
                        break;
                    }
                }
                Err(err) => println!("{:?}", err),
            }
        }
    }
}

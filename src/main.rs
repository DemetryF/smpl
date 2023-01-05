use std::fs;

use lexer::Lexer;

use crate::lexer::token::token_value::TokenValue;

mod lexer;

fn main() {
    if let Ok(code) = fs::read_to_string("test.smpl") {
        println!("{}", code);
        let mut l: Lexer = Lexer::new(code);

        loop {
            let token = l.next_token();

            match token {
                Ok(t) => {
                    println!("{:?}", t);
                    match t.value {
                        TokenValue::EOF => break,
                        _ => (),
                    }
                }
                Err(err) => println!("{:?}", err),
            }
        }
    }
}

use lexer::Lexer;

mod error;
mod lexer;

fn main() {
    let mut lexer = Lexer::new("aboba");

    println!("{:#?}", lexer.next_token().unwrap());
}

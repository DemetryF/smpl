use parser::Parser;

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;

fn main() {
    let code = "if a == a { print(228); /* govnovoz */ }";

    let mut parser = Parser::new(code).unwrap();

    let stmts = match parser.parse() {
        Ok(stmts) => stmts,
        Err(error) => return println!("{error:?}"),
    };

    for stmt in stmts {
        println!("{stmt:?}")
    }
}

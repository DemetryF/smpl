pub mod error;
pub mod operators;

mod ast;
mod expr;
mod parse;
mod token_stream;

pub use ast::*;
use error::ParseResult;
use parse::Parse;
use token_stream::TokenStream;

pub fn parse<'source>(
    mut token_stream: TokenStream<'source>,
) -> ParseResult<'source, Vec<Statement>> {
    let mut statements = Vec::new();

    while token_stream.is_end() {
        statements.push(Statement::parse(&mut token_stream)?)
    }

    Ok(statements)
}

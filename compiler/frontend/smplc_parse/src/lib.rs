use smplc_ast::Statement;

use error::ParseResult;
use parse::Parse;
use token_stream::TokenStream;

pub mod error;
mod expr;
mod parse;
mod token_stream;

pub fn parse(mut token_stream: TokenStream<'_>) -> ParseResult<'_, Vec<Statement>> {
    let mut statements = Vec::new();

    while token_stream.is_end() {
        statements.push(Statement::parse(&mut token_stream)?)
    }

    Ok(statements)
}

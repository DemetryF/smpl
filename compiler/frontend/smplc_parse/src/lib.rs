use smplc_ast::Statement;

use error::ParseResult;
use parse::Parse;
use smplc_lexer::token::Token;
use token_stream::TokenStream;

pub mod error;

mod expr;
mod parse;
mod token_stream;

#[cfg(test)]
mod tests;

pub fn parse<'source, T>(
    mut token_stream: TokenStream<'source, T>,
) -> ParseResult<'source, Vec<Statement>>
where
    T: Iterator<Item = Token<'source>>,
{
    let mut statements = Vec::new();

    while !token_stream.is_end() {
        statements.push(Statement::parse(&mut token_stream)?)
    }

    Ok(statements)
}

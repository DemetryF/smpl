pub mod token_stream;

mod error;
mod parse;
#[cfg(test)]
mod tests;

use error::ParseResult;
use parse::{Parse, TryParse};
use smplc_ast::Declaration;

pub use error::ParseError;
pub use smplc_lexer::LexError;
use smplc_lexer::Token;
pub use token_stream::TokenStream;

pub fn parse<'source, TS>(
    mut token_stream: TokenStream<'source, TS>,
) -> ParseResult<'source, Vec<Declaration<'source>>>
where
    TS: Iterator<Item = Result<Token<'source>, LexError>>,
{
    let mut declarations = Vec::new();

    while !token_stream.is_end() {
        let declaration = Declaration::parse(&mut token_stream)?;

        declarations.push(declaration);
    }

    Ok(declarations)
}

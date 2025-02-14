mod error;
mod parse;
mod token_stream;

#[cfg(test)]
mod tests;

use error::ParseResult;
use parse::{Parse, TryParse};
use smplc_ast::Declaration;

pub use error::ParseError;
pub use smplc_lexer::LexError;
pub use token_stream::TokenStream;

use token_stream::Tokens;

pub fn parse<'source, TS: Tokens<'source>>(
    mut token_stream: TokenStream<'source, TS>,
) -> ParseResult<'source, Vec<Declaration<'source>>> {
    let mut declarations = Vec::new();

    while !token_stream.is_end() {
        let declaration = Declaration::parse(&mut token_stream)?;

        declarations.push(declaration);
    }

    Ok(declarations)
}

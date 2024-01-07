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
pub use token_stream::TokenStream;

pub fn parse(mut token_stream: TokenStream) -> ParseResult<Vec<Declaration>> {
    let mut declarations = Vec::new();

    while !token_stream.is_end() {
        let declaration = Declaration::parse(&mut token_stream)?;

        declarations.push(declaration);
    }

    Ok(declarations)
}

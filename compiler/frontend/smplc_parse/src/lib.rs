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

pub fn parse<'source>(
    mut token_stream: TokenStream<'source>,
) -> ParseResult<'source, Vec<Declaration<'source>>> {
    let mut decls = Vec::new();

    while !token_stream.is_end() {
        let maybe_decl = Declaration::parse(&mut token_stream);

        let decl = maybe_decl?;

        decls.push(decl);
    }

    Ok(decls)
}

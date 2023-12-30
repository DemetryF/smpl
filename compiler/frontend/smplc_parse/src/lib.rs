pub mod token_stream;

mod block;
mod declaration;
mod error;
mod expr;
mod operators;
mod statement;

#[cfg(test)]
mod tests;

use error::ParseResult;
use smplc_ast::Declaration;

pub use error::ParseError;
pub use smplc_lexer::LexError;
pub use token_stream::TokenStream;

pub trait Parse<'source>: Sized {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self>;
}

pub trait TryParse<'source>: Sized {
    fn try_parse(token_stream: &mut TokenStream<'source>) -> Option<Self>;
}

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

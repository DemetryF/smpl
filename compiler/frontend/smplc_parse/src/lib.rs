pub mod token_stream;

mod block;
mod error;
mod expr;
mod operators;
mod statement;
#[cfg(test)]
mod tests;

use error::ParseResult;
use smplc_ast::Statement;

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
) -> ParseResult<'source, Vec<Statement<'source>>> {
    let mut stmts = Vec::new();

    while !token_stream.is_end() {
        let maybe_stmt = Statement::parse(&mut token_stream);

        let stmt = maybe_stmt?;

        stmts.push(stmt);
    }

    Ok(stmts)
}

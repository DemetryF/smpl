pub mod token_stream;

mod block;
mod error;
mod expr;
mod statement;
#[cfg(test)]
mod tests;

use smplc_ast::Statement;

pub use error::ParseError;
pub use smplc_lexer::LexError;
pub use token_stream::TokenStream;

pub trait Parse: Sized {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, ParseError>;
}

pub fn parse(mut token_stream: TokenStream) -> Result<Vec<Statement>, ParseError> {
    let mut stmts = Vec::new();

    while !token_stream.is_end() {
        let maybe_stmt = Statement::parse(&mut token_stream);

        let stmt = maybe_stmt?;

        stmts.push(stmt);
    }

    Ok(stmts)
}

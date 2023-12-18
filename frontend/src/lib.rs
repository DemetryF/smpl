pub mod ast;
pub mod token_stream;

mod error;
#[cfg(test)]
mod tests;

pub use error::ParseError;
pub use smplc_lexer::LexError;

use ast::{Collect, Statement};
use token_stream::TokenStream;

pub fn parse(mut token_stream: TokenStream) -> Result<Vec<Statement>, ParseError> {
    let mut stmts = Vec::new();

    while !token_stream.is_end() {
        let maybe_stmt = Statement::collect(&mut token_stream);

        let stmt = maybe_stmt?;

        stmts.push(stmt);
    }

    Ok(stmts)
}

pub mod ast;

mod error;
mod lexer;
mod token_stream;

#[cfg(test)]
mod tests;

pub use error::Error;

use ast::{Collect, Statement};
use lexer::Lexer;
use token_stream::TokenStream;

pub fn parse(code: &str) -> Result<Vec<Statement>, Error> {
    let tokens = Lexer::new(code).collect::<Result<Vec<_>, _>>()?;

    let mut token_stream = TokenStream::new(tokens);
    let mut stmts = Vec::new();

    while !token_stream.is_end() {
        let maybe_stmt = Statement::collect(&mut token_stream);

        let stmt = maybe_stmt?;

        stmts.push(stmt);
    }

    Ok(stmts)
}

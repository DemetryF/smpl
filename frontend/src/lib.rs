pub mod ast;
pub mod error;

mod lexer;
mod token_stream;

#[cfg(test)]
mod tests;

use ast::{Collect, Statement};
use error::Error;
use token_stream::TokenStream;

pub fn parse(code: &str) -> Result<Vec<Statement>, Vec<Error>> {
    let mut token_stream = TokenStream::new(code)?;
    let mut stmts = Vec::new();

    while !token_stream.is_end() {
        let maybe_stmt = Statement::collect(&mut token_stream);

        let stmt = maybe_stmt.map_err(|err| vec![err])?;

        stmts.push(stmt);
    }

    Ok(stmts)
}

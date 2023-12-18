pub mod ast;

mod error;
mod lexer;
mod token_stream;

#[cfg(test)]
mod tests;

pub use error::ParseError;
pub use lexer::LexError;

use ast::{Collect, Statement};
use lexer::Lexer;
use token_stream::TokenStream;

pub fn lex(code: &str) -> Result<TokenStream, LexError> {
    let lexer = Lexer::new(code);
    let tokens = lexer.collect::<Result<Vec<_>, _>>()?;
    let token_stream = TokenStream::new(tokens);

    Ok(token_stream)
}

pub fn parse(mut token_stream: TokenStream) -> Result<Vec<Statement>, ParseError> {
    let mut stmts = Vec::new();

    while !token_stream.is_end() {
        let maybe_stmt = Statement::collect(&mut token_stream);

        let stmt = maybe_stmt?;

        stmts.push(stmt);
    }

    Ok(stmts)
}

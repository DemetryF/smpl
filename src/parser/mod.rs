use crate::{ast::Statement, error::Error};

use self::{collect::Collect, token_stream::TokenStream};

mod collect;
mod token_stream;

#[cfg(test)]
mod tests;

pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn new(code: &str) -> Result<Self, Vec<Error>> {
        TokenStream::new(code).map(|token_stream| Self { token_stream })
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, Error> {
        let mut stmts = Vec::new();

        while !self.token_stream.is_end() {
            let stmt = Statement::collect(&mut self.token_stream)?;

            stmts.push(stmt);
        }

        Ok(stmts)
    }
}

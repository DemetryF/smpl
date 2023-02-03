use self::{
    collect::Collect, parser_utils::ParserUtils, power_bindings::PowerBinding,
    token_stream::TokenStream,
};
use crate::{ast::Statement, error::*};

mod collect;
mod parser_utils;
mod power_bindings;
mod token_stream;

pub struct Parser<'code> {
    pub token_stream: TokenStream<'code>,
}

impl<'code> Parser<'code> {
    pub fn new(code: &'code str) -> Result<Self> {
        Ok(Self {
            token_stream: TokenStream::new(code)?,
        })
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut stmts = Vec::new();

        while !self.token_stream.is_end() {
            stmts.push(Statement::collect(&mut self.token_stream)?);
        }

        Ok(stmts)
    }
}

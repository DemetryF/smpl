use self::{
    ast::{statement::Statement, Collect},
    token_stream::TokenStream,
};

pub mod ast;
pub mod parser_utils;
pub mod power_bindings;
pub mod token_stream;

pub struct Parser<'code> {
    pub token_stream: TokenStream<'code>,
}

impl<'code> Parser<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            token_stream: TokenStream::new(code),
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stmts = Vec::new();

        while !self.token_stream.is_end() {
            stmts.push(Statement::collect(&mut self.token_stream));
        }

        stmts
    }
}

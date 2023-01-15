use self::{
    ast::Statement, collectors::statement_collector::StatementCollector, token_stream::TokenStream,
};

pub mod ast;
pub mod collectors;
pub mod parser_utils;
pub mod power_bindings;
pub mod token_stream;

pub struct Parser<'code> {
    token_stream: TokenStream<'code>,
}

impl<'code> Parser<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            token_stream: TokenStream::new(code),
        }
    }

    pub fn parse(&mut self) -> Statement<'code> {
        StatementCollector::collect(&mut self.token_stream)
    }
}
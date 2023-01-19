use self::{ast::statement::Statement, ast::Collect, token_stream::TokenStream};

pub mod ast;
pub mod parser_utils;
pub mod power_bindings;
pub mod token_stream;

pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn new(code: String) -> Self {
        Self {
            token_stream: TokenStream::new(code),
        }
    }

    pub fn parse(&mut self) -> Statement {
        Statement::collect(&mut self.token_stream)
    }
}

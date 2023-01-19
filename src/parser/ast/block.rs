use crate::{lexer::token::token_value::TokenValue, parser::token_stream::TokenStream};

use super::{statement::Statement, Collect};

#[derive(Debug)]
pub struct Block(pub Vec<Statement>);

impl Collect for Block {
    fn collect(token_stream: &mut TokenStream) -> Self {
        let mut stmts = Vec::new();

        token_stream.accept(&TokenValue::OpeningBrace);
        while !token_stream.check(&TokenValue::ClosingBrace) {
            stmts.push(Statement::collect(token_stream));
        }
        token_stream.accept(&TokenValue::ClosingBrace);

        Block(stmts)
    }
}

use crate::{lexer::token::token_value::TokenValue, parser::token_stream::TokenStream};

use super::{statement::Statement, Collect};

#[derive(Debug)]
pub struct Block<'code>(pub Vec<Statement<'code>>);

impl<'code> Collect<'code> for Block<'code> {
    fn collect(token_stream: &mut TokenStream<'code>) -> Self {
        let mut stmts = Vec::new();

        token_stream.accept(&TokenValue::OpeningBrace);
        while !token_stream.check(&TokenValue::ClosingBrace) {
            stmts.push(Statement::collect(token_stream));
        }
        token_stream.accept(&TokenValue::ClosingBrace);

        Block(stmts)
    }
}

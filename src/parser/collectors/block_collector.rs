use crate::{
    lexer::token::token_value::TokenValue,
    parser::{ast::Block, token_stream::TokenStream},
};

use super::statement_collector::StatementCollector;

pub struct BlockCollector;
impl BlockCollector {
    pub fn collect<'code>(token_stream: &mut TokenStream<'code>) -> Block<'code> {
        let mut stmts = Vec::new();

        token_stream.accept(&TokenValue::OpeningBrace);
        while !token_stream.check(&TokenValue::ClosingBrace) {
            stmts.push(StatementCollector::collect(token_stream));
        }
        token_stream.accept(&TokenValue::ClosingBrace);

        Block(stmts)
    }
}

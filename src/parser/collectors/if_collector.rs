use crate::{
    lexer::token::token_value::TokenValue,
    parser::{ast::Statement, parser_utils::ParserUtils, token_stream::TokenStream},
};

use super::block_collector::BlockCollector;

pub struct IfStatementCollector;
impl IfStatementCollector {
    pub fn collect<'code>(token_stream: &mut TokenStream<'code>) -> Statement<'code> {
        token_stream.accept(&TokenValue::If);

        let cond = ParserUtils::parenthesis(token_stream);
        let then_body = BlockCollector::collect(token_stream);

        let else_body = if token_stream.check(&TokenValue::Else) {
            token_stream.skip();
            Some(BlockCollector::collect(token_stream))
        } else {
            None
        };

        Statement::If {
            cond,
            then_body,
            else_body,
        }
    }
}

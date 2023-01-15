use crate::{
    lexer::token::token_value::TokenValue,
    parser::{ast::Statement, parser_utils::ParserUtils, token_stream::TokenStream},
};

use super::block_collector::BlockCollector;

pub struct WhileStatementCollector;
impl WhileStatementCollector {
    pub fn collect<'code>(token_stream: &mut TokenStream<'code>) -> Statement<'code> {
        token_stream.accept(&TokenValue::While);

        let cond = ParserUtils::parenthesis(token_stream);
        let body = BlockCollector::collect(token_stream);

        Statement::While { cond, body }
    }
}

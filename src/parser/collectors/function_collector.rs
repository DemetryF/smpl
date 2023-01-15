use crate::{
    lexer::token::token_value::TokenValue,
    parser::{ast::Statement, parser_utils::ParserUtils, token_stream::TokenStream},
};

use super::block_collector::BlockCollector;

pub struct FunctionStatementCollector;
impl FunctionStatementCollector {
    pub fn collect<'code>(token_stream: &mut TokenStream<'code>) -> Statement<'code> {
        token_stream.accept(&TokenValue::Function);

        let id = ParserUtils::id(token_stream);
        let args = Self::args(token_stream);
        let body = BlockCollector::collect(token_stream);

        Statement::Function { id, args, body }
    }

    fn args<'code>(token_stream: &mut TokenStream<'code>) -> Vec<&'code str> {
        let mut args = Vec::new();

        token_stream.accept(&TokenValue::OpeningParen);

        if !token_stream.check(&TokenValue::ClosingParen) {
            args.push(ParserUtils::id(token_stream));

            while token_stream.check(&TokenValue::Comma) {
                token_stream.skip();
                args.push(ParserUtils::id(token_stream));
            }
        }

        token_stream.accept(&TokenValue::ClosingParen);

        args
    }
}

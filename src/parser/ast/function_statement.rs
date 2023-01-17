use derive_more::Constructor;

use crate::{
    lexer::token::token_value::{Id, TokenValue},
    parser::{parser_utils::ParserUtils, token_stream::TokenStream},
};

use super::{block::Block, Collect};

#[derive(Debug, Constructor)]
pub struct FunctionStatement<'code> {
    pub id: Id<'code>,
    pub args: Vec<Id<'code>>,
    pub body: Block<'code>,
}

impl<'code> Collect<'code> for FunctionStatement<'code> {
    fn collect(token_stream: &mut TokenStream<'code>) -> Self {
        token_stream.accept(&TokenValue::Function);

        let id = ParserUtils::id(token_stream);
        let args = Self::args(token_stream);
        let body = Block::collect(token_stream);

        FunctionStatement::new(id, args, body)
    }
}

impl<'code> FunctionStatement<'code> {
    fn args(token_stream: &mut TokenStream<'code>) -> Vec<Id<'code>> {
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

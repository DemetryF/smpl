use derive_more::Constructor;

use crate::{
    lexer::token::token_value::TokenValue,
    parser::{
        ast::{block::Block, Collect},
        parser_utils::ParserUtils,
        token_stream::TokenStream,
    },
};

#[derive(Debug, Constructor)]
pub struct FunctionStatement {
    pub id: String,
    pub args: Vec<String>,
    pub body: Block,
}

impl Collect for FunctionStatement {
    fn collect(token_stream: &mut TokenStream) -> Self {
        token_stream.accept(&TokenValue::Function);

        let id = ParserUtils::id(token_stream);
        let args = Self::args(token_stream);
        let body = Block::collect(token_stream);

        FunctionStatement::new(id, args, body)
    }
}

impl FunctionStatement {
    fn args(token_stream: &mut TokenStream) -> Vec<String> {
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

use crate::{
    lexer::token::token_value::TokenValue,
    parser::{
        ast::{block::Block, Collect, Id},
        parser_utils::ParserUtils,
        token_stream::TokenStream,
    },
};

use super::Statement;

#[derive(Debug)]
pub struct FunctionStatement {
    pub id: Id,
    pub args: Vec<Id>,
    pub body: Block,

    pub has_return: bool,
}

impl Collect for FunctionStatement {
    fn collect(token_stream: &mut TokenStream) -> Self {
        token_stream.accept(&TokenValue::Function);
        token_stream.in_function = true;

        let id = ParserUtils::id(token_stream);
        let args = Self::args(token_stream);
        let body = Block::collect(token_stream);
        token_stream.in_function = false;

        FunctionStatement::new(id, args, body)
    }
}

impl FunctionStatement {
    fn new(id: Id, args: Vec<Id>, body: Block) -> Self {
        let has_return = body
            .0
            .iter()
            .any(|stmt| matches!(stmt, Statement::Return(_)));

        Self {
            id,
            args,
            body,
            has_return,
        }
    }

    fn args(token_stream: &mut TokenStream) -> Vec<Id> {
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

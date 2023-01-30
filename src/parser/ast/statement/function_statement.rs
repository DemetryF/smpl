use crate::{
    lexer::TokenValue,
    parser::{
        ast::{Block, Collect, Id},
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

        let id = ParserUtils::id(token_stream);

        let args = Self::args(token_stream);

        token_stream.in_function = true;
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

        if token_stream.skip_if(&TokenValue::ClosingParen).is_some() {
            return args;
        }

        args.push(ParserUtils::id(token_stream));
        while token_stream.skip_if(&TokenValue::Comma).is_some() {
            args.push(ParserUtils::id(token_stream));
        }

        token_stream.accept(&TokenValue::ClosingParen);

        args
    }
}

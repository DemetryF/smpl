use crate::{
    ast::{Block, FunctionStatement, Id, Statement},
    error::*,
    parser::{Collect, ParserUtils, TokenStream},
    token::TokenValue,
};

impl Collect for FunctionStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        token_stream.accept(&TokenValue::Function)?;

        let id = ParserUtils::id(token_stream)?;
        let args = Self::args(token_stream)?;

        token_stream.in_function = true;
        let body = Block::collect(token_stream)?;
        token_stream.in_function = false;

        Ok(FunctionStatement::new(id, args, body))
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

    fn args(token_stream: &mut TokenStream) -> Result<Vec<Id>> {
        let mut args = Vec::new();

        token_stream.accept(&TokenValue::OpeningParen)?;
        if token_stream.skip_if(&TokenValue::ClosingParen)?.is_some() {
            return Ok(args);
        }

        args.push(ParserUtils::id(token_stream)?);
        while token_stream.skip_if(&TokenValue::Comma)?.is_some() {
            args.push(ParserUtils::id(token_stream)?);
        }

        token_stream.accept(&TokenValue::ClosingParen)?;

        Ok(args)
    }
}

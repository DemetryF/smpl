use smplc_lexer::TokenValue;

use crate::{
    ast::{Block, Collect, Id},
    error::ParseError,
    token_stream::TokenStream,
};

#[derive(PartialEq, Debug)]
pub struct FunctionStatement {
    pub id: Id,
    pub args: Vec<Id>,
    pub body: Block,
}

impl Collect for FunctionStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::Fn)?;

        let id = Id::collect(token_stream)?;
        let args = parse_args(token_stream)?;

        token_stream.in_function = true;
        let body = Block::collect(token_stream)?;
        token_stream.in_function = false;

        Ok(FunctionStatement { id, args, body })
    }
}

fn parse_args(token_stream: &mut TokenStream) -> Result<Vec<Id>, ParseError> {
    let mut args = Vec::new();

    token_stream.consume(TokenValue::LParen)?;

    if token_stream.try_consume(TokenValue::RParen) {
        return Ok(args);
    }

    args.push(Id::collect(token_stream)?);

    while token_stream.try_consume(TokenValue::Comma) {
        args.push(Id::collect(token_stream)?);
    }

    token_stream.consume(TokenValue::RParen)?;

    Ok(args)
}

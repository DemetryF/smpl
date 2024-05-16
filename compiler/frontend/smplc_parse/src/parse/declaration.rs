use smplc_ast::*;
use smplc_lexer::TokenValue;

use crate::error::ParseResult;
use crate::{Parse, TokenStream};

impl<'source> Parse<'source> for Declaration<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        match token_stream.current().value {
            TokenValue::Const => ConstantDeclaration::parse(token_stream).map(Self::Constant),
            TokenValue::Fn => FunctionDeclaration::parse(token_stream).map(Self::Function),

            _ => Err(token_stream.unexpected_token()),
        }
    }
}

impl<'source> Parse<'source> for ConstantDeclaration<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::Const)?;

        let id = Id::parse(token_stream)?;

        token_stream.consume(TokenValue::Assign)?;

        let value = Expr::parse(token_stream)?;

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(Self { id, value })
    }
}

impl<'source> Parse<'source> for FunctionDeclaration<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::Fn)?;

        let id = Id::parse(token_stream)?;
        let args = parse_args(token_stream)?;

        let body = Block::parse(token_stream)?;

        Ok(Self { id, args, body })
    }
}

fn parse_args<'source>(
    token_stream: &mut TokenStream<'source>,
) -> ParseResult<'source, Vec<Id<'source>>> {
    let mut args = Vec::new();

    token_stream.consume(TokenValue::LParen)?;

    if token_stream.try_consume(TokenValue::RParen) {
        return Ok(args);
    }

    args.push(Id::parse(token_stream)?);

    while token_stream.try_consume(TokenValue::Comma) {
        args.push(Id::parse(token_stream)?);
    }

    token_stream.consume(TokenValue::RParen)?;

    Ok(args)
}

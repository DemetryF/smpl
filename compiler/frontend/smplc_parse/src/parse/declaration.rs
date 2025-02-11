use smplc_ast::*;
use smplc_lexer::TokenValue;

use crate::error::ParseResult;
use crate::token_stream::Tokens;
use crate::{Parse, TokenStream};

impl<'source> Parse<'source> for Declaration<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        match token_stream.current().value {
            TokenValue::Const => ConstantDeclaration::parse(token_stream).map(Self::Constant),
            TokenValue::Fn => FunctionDeclaration::parse(token_stream).map(Self::Function),

            _ => Err(token_stream.unexpected_token()),
        }
    }
}

impl<'source> Parse<'source> for ConstantDeclaration<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::Const)?;

        let id = Id::parse(token_stream)?;

        token_stream.consume(TokenValue::Colon)?;

        let ty = Type::parse(token_stream)?;

        token_stream.consume(TokenValue::Assign)?;

        let value = Spanned::<Expr>::parse(token_stream)?;

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(Self { id, ty, value })
    }
}

impl<'source> Parse<'source> for FunctionDeclaration<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::Fn)?;

        let id = Id::parse(token_stream)?;
        let args = parse_args(token_stream)?;

        let ret_ty = {
            token_stream
                .try_consume(TokenValue::Arrow)?
                .then(|| Type::parse(token_stream))
                .transpose()?
        };

        let body = Block::parse(token_stream)?;

        Ok(Self {
            id,
            ret_ty,
            args,
            body,
        })
    }
}

fn parse_args<'source, TS: Tokens<'source>>(
    token_stream: &mut TokenStream<'source, TS>,
) -> ParseResult<'source, Vec<FunctionArg<'source>>> {
    let mut args = Vec::new();

    token_stream.consume(TokenValue::LParen)?;

    if token_stream.try_consume(TokenValue::RParen)? {
        return Ok(args);
    }

    args.push(FunctionArg::parse(token_stream)?);

    while token_stream.try_consume(TokenValue::Comma)? {
        args.push(FunctionArg::parse(token_stream)?);
    }

    token_stream.consume(TokenValue::RParen)?;

    Ok(args)
}

impl<'source> Parse<'source> for FunctionArg<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        let id = Id::parse(token_stream)?;

        token_stream.consume(TokenValue::Colon)?;

        let ty = Type::parse(token_stream)?;

        Ok(FunctionArg { id, ty })
    }
}

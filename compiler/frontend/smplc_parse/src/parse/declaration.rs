use smplc_ast::*;
use smplc_lexer::TokenTag;

use crate::error::ParseResult;
use crate::token_stream::Tokens;
use crate::{Parse, TokenStream};

impl<'source> Parse<'source> for Declaration<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        match token_stream.current().tag {
            TokenTag::Const => ConstantDeclaration::parse(token_stream).map(Self::Constant),
            TokenTag::Fn => FunctionDeclaration::parse(token_stream).map(Self::Function),

            _ => Err(token_stream.unexpected_token()),
        }
    }
}

impl<'source> Parse<'source> for ConstantDeclaration<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        token_stream.consume(TokenTag::Const)?;

        let id = Id::parse(token_stream)?;

        token_stream.consume(TokenTag::Colon)?;

        let ty = Id::parse(token_stream)?;

        token_stream.consume(TokenTag::Assign)?;

        let value = Spanned::<Expr>::parse(token_stream)?;

        token_stream.consume(TokenTag::Semicolon)?;

        Ok(Self { id, ty, value })
    }
}

impl<'source> Parse<'source> for FunctionDeclaration<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        token_stream.consume(TokenTag::Fn)?;

        let id = Id::parse(token_stream)?;
        let args = parse_args(token_stream)?;

        let ret_ty = {
            token_stream
                .try_consume(TokenTag::Arrow)?
                .then(|| Id::parse(token_stream))
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

    token_stream.consume(TokenTag::LParen)?;

    if token_stream.try_consume(TokenTag::RParen)? {
        return Ok(args);
    }

    args.push(FunctionArg::parse(token_stream)?);

    while token_stream.try_consume(TokenTag::Comma)? {
        args.push(FunctionArg::parse(token_stream)?);
    }

    token_stream.consume(TokenTag::RParen)?;

    Ok(args)
}

impl<'source> Parse<'source> for FunctionArg<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        let id = Id::parse(token_stream)?;

        token_stream.consume(TokenTag::Colon)?;

        let ty = Id::parse(token_stream)?;

        Ok(FunctionArg { id, ty })
    }
}

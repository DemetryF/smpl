use smplc_ast::*;
use smplc_lexer::{Token, TokenTag};

use crate::error::ParseResult;
use crate::token_stream::Tokens;
use crate::{Parse, ParseError, TokenStream, TryParse};

impl<'source> Parse<'source> for Spanned<Expr<'source>> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        expr_bp(token_stream, 0)
    }
}

fn expr_bp<'source, TS: Tokens<'source>>(
    token_stream: &mut TokenStream<'source, TS>,
    min_bp: usize,
) -> ParseResult<'source, Spanned<Expr<'source>>> {
    let mut lhs = parse_fact(token_stream)?;

    while let Some(op) = BinOp::try_parse(token_stream) {
        let (l_bp, r_bp) = op.power();

        if l_bp < min_bp {
            break;
        }

        token_stream.next_token()?;

        lhs = {
            let rhs = expr_bp(token_stream, r_bp)?;

            let lhs = Box::new(lhs);
            let rhs = Box::new(rhs);

            let span = Span::unite(lhs.span(), rhs.span());

            Expr::Infix { lhs, op, rhs }.spanned(span)
        };
    }

    Ok(lhs)
}

fn parse_fact<'source, TS: Tokens<'source>>(
    token_stream: &mut TokenStream<'source, TS>,
) -> ParseResult<'source, Spanned<Expr<'source>>> {
    let fact = match token_stream.current() {
        Token {
            tag: TokenTag::Id,
            value,
            span,
        } => {
            token_stream.next_token()?;

            let id = Id::new(value, span);

            if token_stream.try_consume(TokenTag::LParen)? {
                let args = parse_call_args(token_stream)?;
                let span = token_stream.consume(TokenTag::RParen)?;

                let span = Span::unite(id.span(), span);

                Expr::Call(Call { id, args }).spanned(span)
            } else {
                Expr::Atom(Atom::Id(id)).spanned(id.span())
            }
        }

        Token {
            tag: TokenTag::LParen,
            ..
        } => {
            let start_span = token_stream.consume(TokenTag::LParen)?;
            let expr = Spanned::<Expr>::parse(token_stream)?;
            let end_span = token_stream.consume(TokenTag::RParen)?;

            let span = Span::unite(start_span, end_span);

            expr.0.spanned(span)
        }

        Token {
            tag: TokenTag::Literal(ty),
            value,
            ..
        } => {
            let span = token_stream.next_token()?.span;

            Expr::Atom(Atom::Literal(Literal { value, ty })).spanned(span)
        }

        _ => {
            if let Some(op) = UnOp::try_parse(token_stream) {
                let start_span = token_stream.next_token()?.span;

                let (_, r_bp) = op.power();

                let rhs = expr_bp(token_stream, r_bp)?;
                let rhs = Box::new(rhs);

                let span = Span::unite(start_span, rhs.span());

                Expr::Prefix { op, rhs }.spanned(span)
            } else {
                return Err(token_stream.unexpected_token());
            }
        }
    };

    Ok(fact)
}

fn parse_call_args<'source, TS: Tokens<'source>>(
    token_stream: &mut TokenStream<'source, TS>,
) -> ParseResult<'source, Vec<Spanned<Expr<'source>>>> {
    let mut args = Vec::new();

    if token_stream.check(TokenTag::RParen) {
        return Ok(args);
    }

    args.push(Spanned::<Expr>::parse(token_stream)?);

    while token_stream.try_consume(TokenTag::Comma)? {
        args.push(Spanned::<Expr>::parse(token_stream)?);
    }

    Ok(args)
}

impl<'source> Parse<'source> for Id<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        match token_stream.next_token()? {
            Token {
                span,
                tag: TokenTag::Id,
                value,
            } => Ok(Id::new(value, span)),

            token => Err(ParseError::unexpected_token(token)),
        }
    }
}

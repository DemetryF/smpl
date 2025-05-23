use smplc_ast::*;
use smplc_lexer::{Token, TokenTag};

use crate::{error::ParseResult, token_stream::Tokens, Parse, ParseError, TokenStream, TryParse};

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
    let mut lhs = token_stream.work(|token_stream| parse_fact(token_stream))?;

    loop {
        if token_stream.check(TokenTag::Colon) {
            let Spanned(swizzle, swizzle_span) =
                token_stream.work(|token_stream| Swizzle::parse(token_stream))?;

            let span = Span::unite(lhs.span(), swizzle_span);

            lhs = Expr::Swizzle {
                lhs: Box::new(lhs),
                swizzle,
            }
            .spanned(span);
        } else if let Some(op) = BinOp::try_parse(token_stream) {
            let (l_bp, r_bp) = op.power();

            if l_bp < min_bp {
                break;
            }

            token_stream.next_token()?;

            lhs = {
                let lhs = Box::new(lhs);

                let rhs: Spanned<Expr<'_>> = expr_bp(token_stream, r_bp)?;
                let rhs = Box::new(rhs);

                let span = Span::unite(lhs.span(), rhs.span());

                Expr::Infix { lhs, op, rhs }.spanned(span)
            };
        } else {
            break;
        }
    }

    Ok(lhs)
}

fn parse_fact<'source, TS: Tokens<'source>>(
    token_stream: &mut TokenStream<'source, TS>,
) -> ParseResult<'source, Expr<'source>> {
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

                token_stream.consume(TokenTag::RParen)?;

                Expr::Call(Call { id, args })
            } else {
                Expr::Atom(Atom::Id(id))
            }
        }

        Token {
            tag: TokenTag::LParen,
            ..
        } => {
            token_stream.consume(TokenTag::LParen)?;

            let expr = Spanned::<Expr>::parse(token_stream)?;

            token_stream.consume(TokenTag::RParen)?;

            expr.0
        }

        Token {
            tag: TokenTag::Literal(ty),
            value,
            ..
        } => {
            token_stream.next_token()?;

            Expr::Atom(Atom::Literal(Literal { value, ty }))
        }

        _ => {
            if let Some(op) = UnOp::try_parse(token_stream) {
                let (_, r_bp) = op.power();
                token_stream.next_token()?;

                let rhs = expr_bp(token_stream, r_bp)?;
                let rhs = Box::new(rhs);

                Expr::Prefix { op, rhs }
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

impl<'source> Parse<'source> for Swizzle {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        token_stream.consume(TokenTag::Colon)?;

        let token = token_stream.consume(TokenTag::Id)?;

        let combination = token
            .value
            .chars()
            .map(Component::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseError::invalid_swizzle(token.span))?;

        match combination.len() {
            1 => Ok(Swizzle::X1(combination.as_slice().try_into().unwrap())),
            2 => Ok(Swizzle::X2(combination.as_slice().try_into().unwrap())),
            3 => Ok(Swizzle::X3(combination.as_slice().try_into().unwrap())),
            4 => Ok(Swizzle::X4(combination.as_slice().try_into().unwrap())),

            _ => Err(ParseError::invalid_swizzle(token.span)),
        }
    }
}

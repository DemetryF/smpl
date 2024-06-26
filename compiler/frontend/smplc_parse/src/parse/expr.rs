use smplc_ast::{Atom, BinOp, Call, Expr, Id, MakeSpanned, Span, Spanned, UnOp};
use smplc_lexer::{Token, TokenValue};

use crate::error::ParseResult;
use crate::{Parse, ParseError, TokenStream, TryParse};

impl<'source> Parse<'source> for Spanned<Expr<'source>> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        expr_bp(token_stream, 0)
    }
}

fn expr_bp<'source>(
    token_stream: &mut TokenStream<'source>,
    min_bp: usize,
) -> ParseResult<'source, Spanned<Expr<'source>>> {
    let mut lhs = parse_fact(token_stream)?;

    while let Some(op) = BinOp::try_parse(token_stream) {
        let (l_bp, r_bp) = op.power();

        if l_bp < min_bp {
            break;
        }

        token_stream.next_token();

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

fn parse_fact<'source>(
    token_stream: &mut TokenStream<'source>,
) -> ParseResult<'source, Spanned<Expr<'source>>> {
    let fact = match token_stream.current().value {
        TokenValue::Id(_) => {
            let id = Id::parse(token_stream)?;

            if token_stream.try_consume(TokenValue::LParen) {
                let args = parse_call_args(token_stream)?;
                let span = token_stream.consume(TokenValue::RParen)?;

                let span = Span::unite(id.span(), span);

                Expr::Call(Call { id, args }).spanned(span)
            } else {
                Expr::Atom(Atom::Id(id)).spanned(id.span())
            }
        }

        TokenValue::LParen => {
            let start_span = token_stream.consume(TokenValue::LParen)?;
            let expr = Spanned::<Expr>::parse(token_stream)?;
            let end_span = token_stream.consume(TokenValue::RParen)?;

            let span = Span::unite(start_span, end_span);

            expr.0.spanned(span)
        }

        TokenValue::Literal(literal) => {
            let span = token_stream.next_token().span;

            Expr::Atom(Atom::Literal(literal)).spanned(span)
        }

        _ => {
            if let Some(op) = UnOp::try_parse(token_stream) {
                let start_span = token_stream.next_token().span;

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

fn parse_call_args<'source>(
    token_stream: &mut TokenStream<'source>,
) -> ParseResult<'source, Vec<Spanned<Expr<'source>>>> {
    let mut args = Vec::new();

    if token_stream.check(TokenValue::RParen) {
        return Ok(args);
    }

    args.push(Spanned::<Expr>::parse(token_stream)?);

    while token_stream.try_consume(TokenValue::Comma) {
        args.push(Spanned::<Expr>::parse(token_stream)?);
    }

    Ok(args)
}

impl<'source> Parse<'source> for Id<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        match token_stream.next_token() {
            Token {
                span,
                value: TokenValue::Id(value),
            } => Ok(Id::new(value, span)),

            token => Err(ParseError::unexpected_token(token)),
        }
    }
}

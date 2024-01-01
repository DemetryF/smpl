use smplc_ast::{Atom, BinOp, Call, Expr, Id, UnOp};
use smplc_lexer::{Token, TokenValue};

use crate::error::ParseResult;
use crate::{Parse, TokenStream, TryParse};

impl<'source> Parse<'source> for Expr<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        expr_bp(token_stream, 0)
    }
}

fn expr_bp<'source>(
    token_stream: &mut TokenStream<'source>,
    min_bp: usize,
) -> ParseResult<'source, Expr<'source>> {
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

            Expr::Infix { lhs, op, rhs }
        };
    }

    Ok(lhs)
}

fn parse_fact<'source>(
    token_stream: &mut TokenStream<'source>,
) -> ParseResult<'source, Expr<'source>> {
    let fact = match token_stream.current().value {
        TokenValue::Id(_) => {
            let id = Id::parse(token_stream)?;

            if token_stream.check(TokenValue::LParen) {
                parse_call(token_stream, id)?
            } else {
                Expr::Atom(Atom::Id(id))
            }
        }

        TokenValue::LParen => {
            token_stream.consume(TokenValue::LParen)?;
            let expr = Expr::parse(token_stream)?;
            token_stream.consume(TokenValue::RParen)?;

            expr
        }

        TokenValue::Literal(literal) => {
            token_stream.next_token();

            Expr::Atom(Atom::Literal(literal))
        }

        _ => {
            if let Some(op) = UnOp::try_parse(token_stream) {
                token_stream.next_token();

                let (_, r_bp) = op.power();

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

pub fn parse_call<'source>(
    token_stream: &mut TokenStream<'source>,
    id: Id<'source>,
) -> ParseResult<'source, Expr<'source>> {
    let args = parse_call_args(token_stream)?;

    Ok(Expr::Call(Call { id, args }))
}

fn parse_call_args<'source>(
    token_stream: &mut TokenStream<'source>,
) -> ParseResult<'source, Vec<Expr<'source>>> {
    let mut args = Vec::new();

    token_stream.consume(TokenValue::LParen)?;

    if token_stream.try_consume(TokenValue::RParen) {
        return Ok(args);
    }

    args.push(Expr::parse(token_stream)?);
    while token_stream.try_consume(TokenValue::Comma) {
        args.push(Expr::parse(token_stream)?);
    }

    token_stream.consume(TokenValue::RParen)?;

    Ok(args)
}

impl<'source> Parse<'source> for Id<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        match &token_stream.current().value {
            TokenValue::Id(_) => {
                let Token { value, pos } = token_stream.next_token();

                let TokenValue::Id(id) = value else {
                    panic!("kaput");
                };

                Ok(Self::new(id, pos))
            }

            _ => Err(token_stream.unexpected_token()),
        }
    }
}

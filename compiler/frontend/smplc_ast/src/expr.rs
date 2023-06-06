use smplc_token::{Posed, TokenValue};

use crate::{error::ParseResult, parse::Parse, token_stream::TokenStream};

use crate::{
    operators::{BinOp, UnOp},
    Atom, Expr,
};

impl<'source> Parse<'source> for Expr<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        expr_bp(token_stream, 0)
    }
}

fn expr_bp<'source>(
    token_stream: &mut TokenStream<'source>,
    min_bp: u8,
) -> ParseResult<'source, Expr<'source>> {
    let mut lhs = fact(token_stream)?;

    while let Ok(op) = BinOp::try_from(token_stream.current().value) {
        let (l_bp, r_bp) = op.get_bp();

        if l_bp < min_bp {
            break;
        }

        token_stream.next();

        lhs = {
            let rhs = expr_bp(token_stream, r_bp)?;

            let lhs = Box::new(lhs);
            let rhs = Box::new(rhs);

            Expr::Binary { lhs, op, rhs }
        };
    }

    Ok(lhs)
}

fn fact<'source>(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Expr<'source>> {
    fn parenthesis<'source>(
        token_stream: &mut TokenStream<'source>,
    ) -> ParseResult<'source, Expr<'source>> {
        token_stream.consume(TokenValue::LParen)?;
        let expr = Expr::parse(token_stream)?;
        token_stream.consume(TokenValue::RParen)?;

        Ok(expr)
    }

    let value = token_stream.current().value;

    match value {
        TokenValue::LParen => parenthesis(token_stream),

        TokenValue::Ident(id) => {
            let pos = token_stream.next().pos;

            let id = Posed { value: id, pos };

            if token_stream.check(TokenValue::LParen) {
                pub fn parse_call_args<'source>(
                    token_stream: &mut TokenStream<'source>,
                ) -> ParseResult<'source, Vec<Expr<'source>>> {
                    token_stream.consume(TokenValue::LParen)?;

                    let mut args = Vec::new();

                    if token_stream.try_consume(TokenValue::RParen) {
                        return Ok(args);
                    }

                    args.push(Expr::parse(token_stream)?);

                    while token_stream.try_consume(TokenValue::Comma) {
                        args.push(Expr::parse(token_stream)?);
                    }

                    Ok(args)
                }

                let args = parse_call_args(token_stream)?;

                token_stream.consume(TokenValue::RParen)?;

                let expr: Expr = Expr::Call { id, args };

                Ok(expr)
            } else {
                Ok(Expr::Atom(Atom::Ident(id)))
            }
        }

        TokenValue::Literal(literal) => {
            token_stream.next();

            Ok(Expr::Atom(Atom::Literal(literal)))
        }

        value => {
            if let Ok(op) = UnOp::try_from(value) {
                token_stream.next();

                let r_bp = op.get_bp();

                let rhs = expr_bp(token_stream, r_bp)?;
                let rhs = Box::new(rhs);

                Ok(Expr::Unary { op, rhs })
            } else {
                Err(token_stream.unexpected_token())
            }
        }
    }
}

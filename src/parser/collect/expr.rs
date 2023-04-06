use crate::{
    ast::{
        expr::{Atom, Expr},
        id::Id,
        operators::{BinOp, UnOp},
    },
    error::Error,
    lexer::token::TokenValue,
    parser::token_stream::TokenStream,
};

use super::Collect;

impl Collect for Expr {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        expr_bp(token_stream, 0)
    }
}

fn expr_bp(token_stream: &mut TokenStream, min_bp: usize) -> Result<Expr, Error> {
    let mut lhs = parse_fact(token_stream)?;

    while let Ok(op) = BinOp::try_from(token_stream.current()) {
        let (l_bp, r_bp) = op.power();

        if l_bp < min_bp {
            break;
        }

        token_stream.next();

        lhs = {
            let rhs = expr_bp(token_stream, r_bp)?;

            let lhs = Box::new(lhs);
            let rhs = Box::new(rhs);

            Expr::Infix { lhs, op, rhs }
        };

        continue;
    }

    Ok(lhs)
}

fn parse_fact(token_stream: &mut TokenStream) -> Result<Expr, Error> {
    let fact = match token_stream.current().value {
        TokenValue::Id(_) => {
            let id = Id::try_from(token_stream.next()).unwrap();

            if token_stream.check(TokenValue::LParen) {
                parse_call(token_stream, id)?
            } else {
                Expr::Atom(Atom::Id(id))
            }
        }

        TokenValue::LParen => {
            token_stream.consume(TokenValue::LParen)?;
            let expr = Expr::collect(token_stream)?;
            token_stream.consume(TokenValue::RParen)?;

            expr
        }

        TokenValue::Literal(literal) => {
            token_stream.next();

            Expr::Atom(Atom::Literal(literal))
        }

        _ => {
            if let Ok(op) = UnOp::try_from(token_stream.current()) {
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

fn parse_call(token_stream: &mut TokenStream, id: Id) -> Result<Expr, Error> {
    let args = parse_call_args(token_stream)?;

    Ok(Expr::Call { id, args })
}

fn parse_call_args(token_stream: &mut TokenStream) -> Result<Vec<Expr>, Error> {
    let mut args = Vec::new();

    token_stream.consume(TokenValue::LParen)?;

    if token_stream.try_consume(TokenValue::RParen) {
        return Ok(args);
    }

    args.push(Expr::collect(token_stream)?);
    while token_stream.try_consume(TokenValue::Comma) {
        args.push(Expr::collect(token_stream)?);
    }

    token_stream.consume(TokenValue::RParen)?;

    Ok(args)
}

use smplc_ast::{Atom, BinOp, Call, Expr, Id, UnOp};
use smplc_lexer::{Token, TokenValue};

use super::Parse;
use crate::error::ParseError;
use crate::{TokenStream, TryParse};

impl Parse for Expr {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        expr_bp(token_stream, 0)
    }
}

fn expr_bp(token_stream: &mut TokenStream, min_bp: usize) -> Result<Expr, ParseError> {
    let mut lhs = parse_fact(token_stream)?;

    while let Some(op) = BinOp::try_parse(token_stream) {
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

fn parse_fact(token_stream: &mut TokenStream) -> Result<Expr, ParseError> {
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
            token_stream.next();

            Expr::Atom(Atom::Literal(literal))
        }

        _ => {
            if let Some(op) = UnOp::try_parse(token_stream) {
                token_stream.next();

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

pub fn parse_call(token_stream: &mut TokenStream, id: Id) -> Result<Expr, ParseError> {
    let args = parse_call_args(token_stream)?;

    Ok(Expr::Call(Call { id, args }))
}

fn parse_call_args(token_stream: &mut TokenStream) -> Result<Vec<Expr>, ParseError> {
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

impl Parse for Id {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        match &token_stream.current().value {
            TokenValue::Id(_) => {
                let Token { value, pos } = token_stream.next();
                let TokenValue::Id(id) = value else {
                    panic!("kaput");
                };

                Ok(Self::new(id, pos))
            }

            _ => Err(token_stream.unexpected_token()),
        }
    }
}

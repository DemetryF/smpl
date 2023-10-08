use smplc_ast::expr::{Atom, Ident};
use smplc_ast::operators::{BinOp, UnOp};
use smplc_ast::Expr;
use smplc_lexer::token::{Token, TokenValue};

use crate::error::ParseResult;
use crate::parse::Parse;
use crate::token_stream::TokenStream;

impl<'source> Parse<'source> for Expr<'source> {
    fn parse<I>(token_stream: &mut TokenStream<'source, I>) -> ParseResult<'source, Self>
    where
        I: Iterator<Item = Token<'source>>,
    {
        expr_bp(token_stream, 0)
    }
}

fn expr_bp<'source, I>(
    token_stream: &mut TokenStream<'source, I>,
    min_bp: u8,
) -> ParseResult<'source, Expr<'source>>
where
    I: Iterator<Item = Token<'source>>,
{
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

fn fact<'source, I>(
    token_stream: &mut TokenStream<'source, I>,
) -> ParseResult<'source, Expr<'source>>
where
    I: Iterator<Item = Token<'source>>,
{
    let value = token_stream.current().value;

    match value {
        TokenValue::LParen => parenthesis(token_stream),

        TokenValue::Ident(id) => {
            let pos = token_stream.next().pos;

            let id = Ident { value: id, pos };

            if token_stream.check(TokenValue::LParen) {
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

fn parenthesis<'source, I>(
    token_stream: &mut TokenStream<'source, I>,
) -> ParseResult<'source, Expr<'source>>
where
    I: Iterator<Item = Token<'source>>,
{
    token_stream.consume(TokenValue::LParen)?;
    let expr = Expr::parse(token_stream)?;
    token_stream.consume(TokenValue::RParen)?;

    Ok(expr)
}

pub fn parse_call_args<'source, I>(
    token_stream: &mut TokenStream<'source, I>,
) -> ParseResult<'source, Vec<Expr<'source>>>
where
    I: Iterator<Item = Token<'source>>,
{
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

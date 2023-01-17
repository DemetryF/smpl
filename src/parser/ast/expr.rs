use crate::{
    lexer::token::{
        operator::Operator,
        token_value::{Literal, TokenValue},
    },
    parser::{parser_utils::ParserUtils, power_bindings::PowerBinding, token_stream::TokenStream},
};

use super::Collect;

#[derive(Debug)]
pub enum Expr<'code> {
    Binary {
        left: Box<Self>,
        op: Operator,
        right: Box<Expr<'code>>,
    },
    Unary {
        op: Operator,
        expr: Box<Expr<'code>>,
    },
    Call {
        id: &'code str,
        args: Vec<Expr<'code>>,
    },
    Atom(Atom<'code>),
}

#[derive(Debug)]
pub enum Atom<'code> {
    Literal(Literal),
    Id(&'code str),
}

impl<'code> Collect<'code> for Expr<'code> {
    fn collect(token_stream: &mut TokenStream<'code>) -> Self {
        Self::expr_bp(token_stream, 0)
    }
}

impl<'code> Expr<'code> {
    fn expr_bp(token_stream: &mut TokenStream<'code>, bp: u8) -> Self {
        let mut lhs = Self::fact(token_stream);

        while let TokenValue::Operator(op) = token_stream.current().value {
            token_stream.skip();

            if let Some((l_bp, r_bp)) = PowerBinding::infix(op) {
                if l_bp < bp {
                    break;
                }

                lhs = {
                    let rhs = Self::expr_bp(token_stream, r_bp);

                    Self::Binary {
                        left: Box::new(lhs),
                        op,
                        right: Box::new(rhs),
                    }
                };

                continue;
            }

            break;
        }

        lhs
    }

    fn fact(token_stream: &mut TokenStream<'code>) -> Self {
        match token_stream.current().value {
            TokenValue::Literal(literal) => Self::literal(token_stream, literal),
            TokenValue::OpeningParen => ParserUtils::parenthesis(token_stream),
            TokenValue::Operator(op) => Self::unary(token_stream, op),

            TokenValue::Id(id) => {
                if token_stream.following().value == TokenValue::OpeningParen {
                    Self::call(token_stream, id)
                } else {
                    token_stream.skip();
                    Self::Atom(Atom::Id(id))
                }
            }

            t => panic!("bad token: {:?}", t),
        }
    }

    fn unary(token_stream: &mut TokenStream<'code>, op: Operator) -> Self {
        token_stream.skip();

        let ((), r_bp) = PowerBinding::prefix(op);
        let rhs = Self::expr_bp(token_stream, r_bp);

        Self::Unary {
            op,
            expr: Box::new(rhs),
        }
    }

    fn literal(token_stream: &mut TokenStream, literal: Literal) -> Self {
        token_stream.skip();
        Self::Atom(Atom::Literal(literal))
    }

    fn call(token_stream: &mut TokenStream<'code>, id: &'code str) -> Self {
        token_stream.skip();
        let args = Self::call_args(token_stream);

        Self::Call { id, args }
    }

    fn call_args(token_stream: &mut TokenStream<'code>) -> Vec<Self> {
        let mut args = Vec::new();

        token_stream.accept(&TokenValue::OpeningParen);

        if !token_stream.check(&TokenValue::ClosingParen) {
            args.push(Self::collect(token_stream));

            while token_stream.check(&TokenValue::Comma) {
                token_stream.skip();
                args.push(Self::collect(token_stream));
            }
        }

        token_stream.accept(&TokenValue::ClosingParen);

        args
    }
}

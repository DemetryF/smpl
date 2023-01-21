use derive_more::Constructor;

use crate::{
    lexer::token::{
        operator::Operator,
        token_value::{Literal, TokenValue},
    },
    parser::{parser_utils::ParserUtils, power_bindings::PowerBinding, token_stream::TokenStream},
};

use self::{call::Call, unary::Unary};

use super::Collect;

pub mod call;
pub mod unary;

#[derive(Debug)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Call(Call),
    Atom(Atom),
}

#[derive(Debug, Constructor)]
pub struct Binary {
    pub lhs: Box<Expr>,
    pub op: Operator,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub enum Atom {
    Literal(Literal),
    Id(String),
}

impl Collect for Expr {
    fn collect(token_stream: &mut TokenStream) -> Self {
        Self::expr_bp(token_stream, 0)
    }
}

impl Expr {
    pub fn expr_bp(token_stream: &mut TokenStream, bp: u8) -> Self {
        let mut lhs = Self::fact(token_stream);

        while let TokenValue::Operator(op) = token_stream.current().value {
            if let Some((l_bp, r_bp)) = PowerBinding::infix(op) {
                if l_bp < bp {
                    break;
                }
                token_stream.skip();

                lhs = {
                    let rhs = Self::expr_bp(token_stream, r_bp);

                    Self::Binary(Binary::new(Box::new(lhs), op, Box::new(rhs)))
                };

                continue;
            }

            break;
        }

        lhs
    }

    fn fact(token_stream: &mut TokenStream) -> Self {
        match token_stream.current().value.clone() {
            TokenValue::Literal(literal) => Self::literal(token_stream, literal),
            TokenValue::OpeningParen => ParserUtils::parenthesis(token_stream),
            TokenValue::Operator(_) => Self::Unary(Unary::collect(token_stream)),

            TokenValue::Id(id) => {
                if token_stream.following().value == TokenValue::OpeningParen {
                    Self::Call(Call::collect(token_stream))
                } else {
                    token_stream.skip();
                    Self::Atom(Atom::Id(id))
                }
            }

            t => panic!("bad token: {:?}", t),
        }
    }

    fn literal(token_stream: &mut TokenStream, literal: Literal) -> Self {
        token_stream.skip();
        Self::Atom(Atom::Literal(literal))
    }
}

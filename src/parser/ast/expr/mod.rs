use derive_more::Constructor;

pub use self::{call::Call, unary::Unary};

use crate::{
    error::*,
    lexer::{Literal, Operator, TokenValue},
    parser::{
        ast::{Collect, Id},
        PowerBinding, TokenStream,
    },
};

pub mod call;
pub mod unary;

pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Call(Call),
    Atom(Atom),
}

#[derive(Constructor)]
pub struct Binary {
    pub lhs: Box<Expr>,
    pub op: Operator,
    pub rhs: Box<Expr>,
}

#[derive(Clone)]
pub enum Atom {
    Literal(Literal),
    Temp(usize),
    Id(Id),
}

impl Collect for Expr {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        Self::expr_bp(token_stream, 0)
    }
}

impl Expr {
    pub fn expr_bp(token_stream: &mut TokenStream, bp: u8) -> Result<Self> {
        let mut lhs = Self::fact(token_stream)?;

        while let TokenValue::Operator(op) = token_stream.current().value {
            if let Some((l_bp, r_bp)) = PowerBinding::infix(op) {
                if l_bp < bp {
                    break;
                }
                token_stream.skip();

                lhs = {
                    let rhs = Self::expr_bp(token_stream, r_bp)?;

                    Self::Binary(Binary::new(Box::new(lhs), op, Box::new(rhs)))
                };

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn fact(token_stream: &mut TokenStream) -> Result<Self> {
        let token = token_stream.current().clone();

        Ok(match token.value.clone() {
            TokenValue::Literal(literal) => Self::literal(token_stream, literal),
            TokenValue::OpeningParen => Self::parenthesis(token_stream)?,
            TokenValue::Operator(_) => Self::Unary(Unary::collect(token_stream)?),

            TokenValue::Id(id) => {
                if token_stream.following().value == TokenValue::OpeningParen {
                    Self::Call(Call::collect(token_stream)?)
                } else {
                    token_stream.skip();
                    Self::Atom(Atom::Id(Id::new(id, token.pos)))
                }
            }

            _ => return Err(Error::UnexpectedToken(token.clone())),
        })
    }

    fn literal(token_stream: &mut TokenStream, literal: Literal) -> Self {
        token_stream.skip();
        Self::Atom(Atom::Literal(literal))
    }

    pub fn parenthesis(token_stream: &mut TokenStream) -> Result<Expr> {
        token_stream.accept(&TokenValue::OpeningParen);
        let expr = Self::collect(token_stream)?;
        token_stream.accept(&TokenValue::ClosingParen);

        Ok(expr)
    }
}

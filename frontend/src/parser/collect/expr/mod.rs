use crate::{
    ast::{Atom, Call, Expr, Id, Infix, Prefix},
    error::*,
    parser::{Collect, PowerBinding, TokenStream},
    token::{Literal, TokenValue},
};

pub mod call;
pub mod prefix;

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
                token_stream.skip()?;

                lhs = {
                    let rhs = Self::expr_bp(token_stream, r_bp)?;

                    Self::Infix(Infix::new(Box::new(lhs), op, Box::new(rhs)))
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
            TokenValue::Literal(literal) => Self::literal(token_stream, literal)?,
            TokenValue::OpeningParen => Self::parenthesis(token_stream)?,
            TokenValue::Operator(_) => Self::Prefix(Prefix::collect(token_stream)?),

            TokenValue::Id(id) => {
                if token_stream.following()?.value == TokenValue::OpeningParen {
                    Self::Call(Call::collect(token_stream)?)
                } else {
                    token_stream.skip()?;
                    Self::Atom(Atom::Id(Id::new(id, token.pos)))
                }
            }

            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken(token.value),
                    token.pos,
                ))
            }
        })
    }

    fn literal(token_stream: &mut TokenStream, literal: Literal) -> Result<Self> {
        token_stream.skip()?;
        Ok(Self::Atom(Atom::Literal(literal)))
    }

    pub fn parenthesis(token_stream: &mut TokenStream) -> Result<Expr> {
        token_stream.accept(&TokenValue::OpeningParen)?;
        let expr = Self::collect(token_stream)?;
        token_stream.accept(&TokenValue::ClosingParen)?;

        Ok(expr)
    }
}

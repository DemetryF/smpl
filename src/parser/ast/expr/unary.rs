use derive_more::Constructor;

use crate::{
    error::*,
    lexer::Operator,
    parser::{
        ast::{Collect, Expr},
        ParserUtils, PowerBinding, TokenStream,
    },
};

#[derive(Constructor)]
pub struct Unary {
    pub op: Operator,
    pub rhs: Box<Expr>,
}

impl Collect for Unary {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        let op = ParserUtils::op(token_stream)?;
        let r_bp = PowerBinding::prefix(op);
        let rhs = Expr::expr_bp(token_stream, r_bp)?;

        Ok(Self::new(op, Box::new(rhs)))
    }
}

use derive_more::Constructor;

use crate::{
    lexer::token::operator::Operator,
    parser::{
        ast::Collect, parser_utils::ParserUtils, power_bindings::PowerBinding,
        token_stream::TokenStream,
    },
};

use super::Expr;

#[derive(Debug, Constructor)]
pub struct Unary {
    pub op: Operator,
    pub rhs: Box<Expr>,
}

impl Collect for Unary {
    fn collect(token_stream: &mut TokenStream) -> Self {
        let op = ParserUtils::op(token_stream);
        let ((), r_bp) = PowerBinding::prefix(op);
        let rhs = Expr::expr_bp(token_stream, r_bp);

        Self::new(op, Box::new(rhs))
    }
}

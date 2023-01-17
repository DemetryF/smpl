use derive_more::Constructor;

use crate::{
    lexer::token::{operator::Operator, token_value::TokenValue},
    parser::{parser_utils::ParserUtils, token_stream::TokenStream},
};

use super::{expr::Expr, Collect};

#[derive(Debug, Constructor)]
pub struct DeclareStatement<'code> {
    pub id: &'code str,
    pub expr: Option<Expr<'code>>,
}

impl<'code> Collect<'code> for DeclareStatement<'code> {
    fn collect(token_stream: &mut TokenStream<'code>) -> Self {
        token_stream.accept(&TokenValue::Define);

        let id = ParserUtils::id(token_stream);
        let expr = Self::init_expr(token_stream);

        token_stream.accept(&TokenValue::Semicolon);

        DeclareStatement::new(id, expr)
    }
}

impl<'code> DeclareStatement<'code> {
    fn init_expr(token_stream: &mut TokenStream<'code>) -> Option<Expr<'code>> {
        if token_stream.check(&TokenValue::Operator(Operator::Assignment)) {
            token_stream.skip();
            Some(Expr::collect(token_stream))
        } else {
            None
        }
    }
}

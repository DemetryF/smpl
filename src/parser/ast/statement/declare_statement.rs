use derive_more::Constructor;

use crate::{
    lexer::token::{operator::Operator, token_value::TokenValue},
    parser::{ast::Id, parser_utils::ParserUtils, token_stream::TokenStream},
};

use super::super::{expr::Expr, Collect};

#[derive(Debug, Constructor)]
pub struct DeclareStatement {
    pub id: Id,
    pub expr: Option<Expr>,
}

impl Collect for DeclareStatement {
    fn collect(token_stream: &mut TokenStream) -> Self {
        token_stream.accept(&TokenValue::Define);

        let id = ParserUtils::id(token_stream);
        let expr = Self::init_expr(token_stream);

        token_stream.accept(&TokenValue::Semicolon);

        DeclareStatement::new(id, expr)
    }
}

impl DeclareStatement {
    fn init_expr(token_stream: &mut TokenStream) -> Option<Expr> {
        if token_stream.check(&TokenValue::Operator(Operator::Assignment)) {
            token_stream.skip();
            Some(Expr::collect(token_stream))
        } else {
            None
        }
    }
}

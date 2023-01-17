use derive_more::Constructor;

use crate::{lexer::token::token_value::TokenValue, parser::token_stream::TokenStream};

use super::{expr::Expr, Collect};

#[derive(Debug, Constructor)]
pub struct ReturnStatement<'code>(Option<Expr<'code>>);

impl<'code> Collect<'code> for ReturnStatement<'code> {
    fn collect(token_stream: &mut TokenStream<'code>) -> Self {
        token_stream.accept(&TokenValue::Return);

        let expr = Self::return_expr(token_stream);

        token_stream.accept(&TokenValue::Semicolon);

        ReturnStatement::new(expr)
    }
}

impl<'code> ReturnStatement<'code> {
    pub fn return_expr(token_stream: &mut TokenStream<'code>) -> Option<Expr<'code>> {
        if token_stream.check(&TokenValue::Semicolon) {
            None
        } else {
            Some(Expr::collect(token_stream))
        }
    }
}

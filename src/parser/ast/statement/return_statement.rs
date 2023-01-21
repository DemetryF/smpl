use derive_more::Constructor;

use crate::{
    lexer::token::token_value::TokenValue,
    parser::{
        ast::{expr::Expr, Collect},
        token_stream::TokenStream,
    },
};

#[derive(Debug, Constructor)]
pub struct ReturnStatement(pub Option<Expr>);

impl Collect for ReturnStatement {
    fn collect(token_stream: &mut TokenStream) -> Self {
        token_stream.accept(&TokenValue::Return);

        let expr = Self::return_expr(token_stream);

        token_stream.accept(&TokenValue::Semicolon);

        ReturnStatement::new(expr)
    }
}

impl ReturnStatement {
    pub fn return_expr(token_stream: &mut TokenStream) -> Option<Expr> {
        if token_stream.check(&TokenValue::Semicolon) {
            None
        } else {
            Some(Expr::collect(token_stream))
        }
    }
}

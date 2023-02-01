use derive_more::Constructor;

use crate::{
    error::*,
    lexer::TokenValue,
    parser::{
        ast::{Collect, Expr},
        TokenStream,
    },
};

pub struct ReturnStatement(pub Option<Expr>);

impl Collect for ReturnStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        Self::check_in_function(token_stream);

        token_stream.accept(&TokenValue::Return);
        let expr = Self::return_expr(token_stream)?;
        token_stream.accept(&TokenValue::Semicolon);

        Ok(ReturnStatement(expr))
    }
}

impl ReturnStatement {
    pub fn return_expr(token_stream: &mut TokenStream) -> Result<Option<Expr>> {
        if token_stream.check(&TokenValue::Semicolon) {
            Ok(None)
        } else {
            Ok(Some(Expr::collect(token_stream)?))
        }
    }

    pub fn check_in_function(token_stream: &TokenStream) {
        if !token_stream.in_function {
            panic!("use return outside function");
        }
    }
}

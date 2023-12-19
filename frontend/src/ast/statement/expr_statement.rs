use smplc_ast::{Expr, ExprStatement};
use smplc_lexer::TokenValue;

use crate::{ast::Collect, error::ParseError, token_stream::TokenStream};

impl Collect for ExprStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        let expr = Expr::collect(token_stream)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ExprStatement(expr))
    }
}

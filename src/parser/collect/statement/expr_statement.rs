use crate::{
    ast::{expr::Expr, ExprStatement},
    error::Error,
    lexer::token::TokenValue,
    parser::{collect::Collect, token_stream::TokenStream},
};

impl Collect for ExprStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let expr = Expr::collect(token_stream)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ExprStatement(expr))
    }
}

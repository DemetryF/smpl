use crate::{
    ast::{Collect, Expr},
    error::Error,
    lexer::TokenValue,
    token_stream::TokenStream,
};

#[derive(PartialEq, Debug)]
pub struct ExprStatement(pub Expr);

impl Collect for ExprStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let expr = Expr::collect(token_stream)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ExprStatement(expr))
    }
}

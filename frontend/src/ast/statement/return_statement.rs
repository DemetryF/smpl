use crate::{
    ast::{Collect, Expr},
    error::Error,
    lexer::TokenValue,
    token_stream::TokenStream,
};

#[derive(PartialEq, Debug)]
pub struct ReturnStatement(pub Option<Expr>);

impl Collect for ReturnStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        check_in_function(token_stream)?;

        token_stream.consume(TokenValue::Return)?;
        let expr = return_expr(token_stream)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ReturnStatement(expr))
    }
}

pub fn return_expr(token_stream: &mut TokenStream) -> Result<Option<Expr>, Error> {
    let maybe_expr = if token_stream.check(TokenValue::Semicolon) {
        None
    } else {
        let expr = Expr::collect(token_stream)?;

        Some(expr)
    };

    Ok(maybe_expr)
}

pub fn check_in_function(token_stream: &TokenStream) -> Result<(), Error> {
    if !token_stream.in_function {
        let error = Error::return_outside_function(token_stream.get_pos());

        return Err(error);
    }

    Ok(())
}

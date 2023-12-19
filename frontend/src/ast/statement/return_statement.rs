use smplc_ast::{Expr, ReturnStatement};
use smplc_lexer::TokenValue;

use crate::{ast::Collect, error::ParseError, token_stream::TokenStream};

impl Collect for ReturnStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        check_in_function(token_stream)?;

        token_stream.consume(TokenValue::Return)?;
        let expr = return_expr(token_stream)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ReturnStatement(expr))
    }
}

fn return_expr(token_stream: &mut TokenStream) -> Result<Option<Expr>, ParseError> {
    let maybe_expr = if token_stream.check(TokenValue::Semicolon) {
        None
    } else {
        let expr = Expr::collect(token_stream)?;

        Some(expr)
    };

    Ok(maybe_expr)
}

fn check_in_function(token_stream: &TokenStream) -> Result<(), ParseError> {
    if !token_stream.in_function {
        let error = ParseError::return_outside_function(token_stream.get_pos());

        return Err(error);
    }

    Ok(())
}

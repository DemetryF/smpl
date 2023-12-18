use crate::{
    ast::{Collect, Expr, Id},
    error::ParseError,
    lexer::TokenValue,
    TokenStream,
};

#[derive(Debug, PartialEq)]
pub struct DeclareStatement {
    pub id: Id,
    pub init_expr: Option<Expr>,
}

impl Collect for DeclareStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::Let)?;

        let id = Id::collect(token_stream)?;
        let init_expr = parse_init_expr(token_stream)?;

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(DeclareStatement { id, init_expr })
    }
}

fn parse_init_expr(token_stream: &mut TokenStream) -> Result<Option<Expr>, ParseError> {
    if token_stream.try_consume(TokenValue::Assignment) {
        let expr = Expr::collect(token_stream)?;

        return Ok(Some(expr));
    }

    Ok(None)
}

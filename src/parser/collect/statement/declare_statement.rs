use crate::{
    ast::{expr::Expr, id::Id, DeclareStatement},
    error::Error,
    lexer::token::TokenValue,
    parser::{collect::Collect, token_stream::TokenStream},
};

impl Collect for DeclareStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        token_stream.consume(TokenValue::Let)?;

        let id = Id::collect(token_stream)?;
        println!("autobus");
        let init_expr = parse_init_expr(token_stream)?;

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(DeclareStatement { id, init_expr })
    }
}

fn parse_init_expr(token_stream: &mut TokenStream) -> Result<Option<Expr>, Error> {
    if token_stream.try_consume(TokenValue::Assignment) {
        let expr = Expr::collect(token_stream)?;

        return Ok(Some(expr));
    }

    Ok(None)
}

use crate::{
    ast::{Collect, Expr, Id},
    error::ParseError,
    lexer::TokenValue,
    token_stream::TokenStream,
};

#[derive(Debug, PartialEq)]
pub struct Call {
    pub id: Id,
    pub args: Vec<Expr>,
}

impl Call {
    pub fn collect(token_stream: &mut TokenStream, id: Id) -> Result<Expr, ParseError> {
        let args = Self::collect_args(token_stream)?;

        Ok(Expr::Call(Call { id, args }))
    }

    fn collect_args(token_stream: &mut TokenStream) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();

        token_stream.consume(TokenValue::LParen)?;

        if token_stream.try_consume(TokenValue::RParen) {
            return Ok(args);
        }

        args.push(Expr::collect(token_stream)?);
        while token_stream.try_consume(TokenValue::Comma) {
            args.push(Expr::collect(token_stream)?);
        }

        token_stream.consume(TokenValue::RParen)?;

        Ok(args)
    }
}

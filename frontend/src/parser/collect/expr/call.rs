use crate::{
    ast::Call,
    error::*,
    parser::{Collect, ParserUtils, TokenStream},
    token::TokenValue,
};

use super::Expr;

impl Collect for Call {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        let id = ParserUtils::id(token_stream)?;
        let args = Self::call_args(token_stream)?;

        Ok(Self::new(id, args))
    }
}

impl Call {
    fn call_args(token_stream: &mut TokenStream) -> Result<Vec<Expr>> {
        let mut args = Vec::new();

        token_stream.accept(&TokenValue::OpeningParen)?;
        if token_stream.skip_if(&TokenValue::ClosingParen)?.is_some() {
            return Ok(args);
        }

        args.push(Expr::collect(token_stream)?);
        while token_stream.skip_if(&TokenValue::Comma)?.is_some() {
            args.push(Expr::collect(token_stream)?);
        }

        token_stream.accept(&TokenValue::ClosingParen)?;

        Ok(args)
    }
}

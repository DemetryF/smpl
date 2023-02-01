use crate::{
    ast::{DeclareStatement, Expr},
    error::*,
    lexer::{Operator, TokenValue},
    parser::{Collect, ParserUtils, TokenStream},
};

impl Collect for DeclareStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        token_stream.accept(&TokenValue::Define)?;

        let id = ParserUtils::id(token_stream)?;
        let expr = Self::init_expr(token_stream)?;

        token_stream.accept(&TokenValue::Semicolon)?;

        Ok(DeclareStatement::new(id, expr))
    }
}

impl DeclareStatement {
    fn init_expr(token_stream: &mut TokenStream) -> Result<Option<Expr>> {
        if token_stream
            .skip_if(&TokenValue::Operator(Operator::Assignment))?
            .is_none()
        {
            Ok(None)
        } else {
            Ok(Some(Expr::collect(token_stream)?))
        }
    }
}

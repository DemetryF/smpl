use crate::{
    ast::{Block, Expr, WhileStatement},
    error::*,
    parser::{Collect, TokenStream},
    token::TokenValue,
};

impl Collect for WhileStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        token_stream.accept(&TokenValue::While)?;

        let cond = Expr::collect(token_stream)?;
        let body = Block::collect(token_stream)?;

        Ok(WhileStatement::new(cond, body))
    }
}

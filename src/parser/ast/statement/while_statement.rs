use derive_more::Constructor;

use crate::{
    error::*,
    lexer::TokenValue,
    parser::{
        ast::{Block, Collect, Expr},
        TokenStream,
    },
};

#[derive(Constructor)]
pub struct WhileStatement {
    pub cond: Expr,
    pub body: Block,
}

impl Collect for WhileStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        token_stream.accept(&TokenValue::While);

        let cond = Expr::collect(token_stream)?;
        let body = Block::collect(token_stream)?;

        Ok(WhileStatement::new(cond, body))
    }
}

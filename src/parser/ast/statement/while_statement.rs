use derive_more::Constructor;

use crate::{
    lexer::TokenValue,
    parser::{
        ast::{Block, Collect, Expr},
        token_stream::TokenStream,
    },
};

#[derive(Debug, Constructor)]
pub struct WhileStatement {
    pub cond: Expr,
    pub body: Block,
}

impl Collect for WhileStatement {
    fn collect(token_stream: &mut TokenStream) -> Self {
        token_stream.accept(&TokenValue::While);

        let cond = Expr::collect(token_stream);
        let body = Block::collect(token_stream);

        WhileStatement::new(cond, body)
    }
}

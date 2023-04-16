use crate::{
    ast::{Block, Collect, Expr},
    error::Error,
    lexer::TokenValue,
    TokenStream,
};

#[derive(PartialEq, Debug)]
pub struct WhileStatement {
    pub condition: Expr,
    pub body: Block,
}

impl Collect for WhileStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        token_stream.consume(TokenValue::While)?;

        let condition = Expr::collect(token_stream)?;
        let body = Block::collect(token_stream)?;

        Ok(WhileStatement { condition, body })
    }
}

use crate::{
    ast::{expr::Expr, Block, WhileStatement},
    error::Error,
    lexer::token::TokenValue,
    parser::{collect::Collect, token_stream::TokenStream},
};

impl Collect for WhileStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        token_stream.consume(TokenValue::While)?;

        let condition = Expr::collect(token_stream)?;
        let body = Block::collect(token_stream)?;

        Ok(WhileStatement { condition, body })
    }
}

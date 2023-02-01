use crate::{
    ast::{Block, Expr, IfStatement},
    error::*,
    lexer::TokenValue,
    parser::{Collect, TokenStream},
};

impl Collect for IfStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        token_stream.accept(&TokenValue::If)?;

        let cond = Expr::collect(token_stream)?;
        let then_body = Block::collect(token_stream)?;
        let else_body = Self::parse_else_body(token_stream)?;

        Ok(IfStatement::new(cond, then_body, else_body))
    }
}

impl IfStatement {
    fn parse_else_body(token_stream: &mut TokenStream) -> Result<Option<Block>> {
        Ok(if token_stream.check(&TokenValue::Else) {
            token_stream.skip()?;
            Some(Block::collect(token_stream)?)
        } else {
            None
        })
    }
}

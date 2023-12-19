use smplc_ast::{Block, Expr, WhileStatement};
use smplc_lexer::TokenValue;

use crate::{ast::Collect, error::ParseError, TokenStream};

impl Collect for WhileStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::While)?;

        let condition = Expr::collect(token_stream)?;
        let body = Block::collect(token_stream)?;

        Ok(WhileStatement { condition, body })
    }
}

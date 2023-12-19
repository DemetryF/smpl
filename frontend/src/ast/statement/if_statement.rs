use smplc_ast::{Block, Expr, IfStatement};
use smplc_lexer::TokenValue;

use crate::{ast::Collect, error::ParseError, token_stream::TokenStream};

impl Collect for IfStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::If)?;

        let condition = Expr::collect(token_stream)?;
        let then_body = Block::collect(token_stream)?;
        let else_body = parse_else_body(token_stream)?;

        Ok(IfStatement {
            condition,
            then_body,
            else_body,
        })
    }
}

fn parse_else_body(token_stream: &mut TokenStream) -> Result<Option<Block>, ParseError> {
    let else_body = if token_stream.try_consume(TokenValue::Else) {
        let block = Block::collect(token_stream)?;

        Some(block)
    } else {
        None
    };

    Ok(else_body)
}

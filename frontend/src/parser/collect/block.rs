use super::{Collect, TokenStream};
use crate::{
    ast::{Block, Statement},
    error::*,
    token::TokenValue,
};

impl Collect for Block {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        let mut stmts = Vec::new();

        token_stream.accept(&TokenValue::OpeningBrace)?;
        while !token_stream.check(&TokenValue::ClosingBrace) {
            let new_stmt = Statement::collect(token_stream)?;

            if matches!(new_stmt, Statement::Function(_)) {
                panic!("not allowed function declare a function at a block")
            }

            stmts.push(new_stmt);
        }
        token_stream.accept(&TokenValue::ClosingBrace)?;

        Ok(Block(stmts))
    }
}

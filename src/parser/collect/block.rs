use crate::{
    ast::{Block, Statement},
    error::Error,
    lexer::token::TokenValue,
    parser::token_stream::TokenStream,
};

use super::Collect;

impl Collect for Block {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let mut stmts = Vec::new();

        token_stream.consume(TokenValue::LBrace)?;
        while !token_stream.check(TokenValue::RBrace) {
            let next_stmt = Statement::collect(token_stream)?;

            if matches!(next_stmt, Statement::Function(_)) {
                let error = Error::function_in_block(token_stream.get_pos());

                return Err(error);
            }

            stmts.push(next_stmt);
        }
        token_stream.consume(TokenValue::RBrace)?;

        Ok(Block { stmts })
    }
}

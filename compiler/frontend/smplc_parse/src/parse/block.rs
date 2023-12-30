use smplc_ast::{Block, Statement};
use smplc_lexer::TokenValue;

use crate::error::ParseResult;
use crate::{Parse, TokenStream};

impl<'source> Parse<'source> for Block<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        let mut stmts = Vec::new();

        token_stream.consume(TokenValue::LBrace)?;

        while !token_stream.check(TokenValue::RBrace) {
            let next_stmt = Statement::parse(token_stream)?;

            stmts.push(next_stmt);
        }

        token_stream.consume(TokenValue::RBrace)?;

        Ok(Block { stmts })
    }
}

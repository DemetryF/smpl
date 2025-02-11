use smplc_ast::{Block, Statement};
use smplc_lexer::TokenValue;

use crate::error::ParseResult;
use crate::token_stream::Tokens;
use crate::{Parse, TokenStream};

impl<'source> Parse<'source> for Block<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        let mut statements = Vec::new();

        token_stream.consume(TokenValue::LBrace)?;

        while !token_stream.check(TokenValue::RBrace) {
            let next_stmt = Statement::parse(token_stream)?;

            statements.push(next_stmt);
        }

        token_stream.consume(TokenValue::RBrace)?;

        Ok(Block { statements })
    }
}

use smplc_ast::{Block, Statement};
use smplc_lexer::TokenValue;

use crate::error::{ParseError, ParseResult};
use crate::TokenStream;

use super::Parse;

impl<'source> Parse<'source> for Block<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        let mut stmts = Vec::new();

        token_stream.consume(TokenValue::LBrace)?;

        while !token_stream.check(TokenValue::RBrace) {
            let next_stmt = Statement::parse(token_stream)?;

            if matches!(next_stmt, Statement::Function(_)) {
                return Err(ParseError::function_in_block(token_stream.get_pos()));
            }

            stmts.push(next_stmt);
        }

        token_stream.consume(TokenValue::RBrace)?;

        Ok(Block { stmts })
    }
}

use super::token_stream::TokenStream;
use crate::{
    ast::Id,
    error::*,
    lexer::{Operator, TokenValue},
};

pub struct ParserUtils;
impl ParserUtils {
    pub fn id(token_stream: &mut TokenStream) -> Result<Id> {
        let token = token_stream.skip()?;
        match token.value {
            TokenValue::Id(value) => Ok(Id::new(value, token.pos)),
            _ => Err(Error::UnexpectedToken(token)),
        }
    }

    pub fn op(token_stream: &mut TokenStream) -> Result<Operator> {
        let token = token_stream.skip()?;
        match token.value {
            TokenValue::Operator(op) => Ok(op),
            _ => Err(Error::UnexpectedToken(token)),
        }
    }
}

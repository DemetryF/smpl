use super::{ast::Id, token_stream::TokenStream};
use crate::lexer::{Operator, TokenValue};

pub struct ParserUtils;
impl ParserUtils {
    pub fn id(token_stream: &mut TokenStream) -> Id {
        let token = token_stream.skip();
        match token.value {
            TokenValue::Id(value) => Id::new(value, token.pos),
            _ => panic!("expected id"),
        }
    }

    pub fn op(token_stream: &mut TokenStream) -> Operator {
        match token_stream.skip().value {
            TokenValue::Operator(op) => op,
            _ => panic!("expected operator"),
        }
    }
}

use strum::IntoEnumIterator;

use crate::{
    lexer::{CodeStream, TokenCollector},
    token::{Operator, TokenValue},
};

pub struct OperatorCollector;
impl TokenCollector for OperatorCollector {
    fn try_next(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        for op in Operator::iter() {
            let value = <&str>::from(op);

            if code_stream.check(value) {
                code_stream.skip(value.len());
                return Some(TokenValue::Operator(op));
            }
        }

        None
    }
}

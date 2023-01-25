use crate::lexer::{
    code_stream::CodeStream,
    token::{operator::Operator, token_value::TokenValue},
    token_collector::TokenCollector,
};

pub struct OperatorCollector;

impl TokenCollector for OperatorCollector {
    fn try_next(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        for op in Operator::all() {
            let value = <&str>::from(op);

            if code_stream.check(value) {
                code_stream.skip(value.len());
                return Some(TokenValue::Operator(op));
            }
        }

        None
    }
}

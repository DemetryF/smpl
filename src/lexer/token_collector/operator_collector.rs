use crate::lexer::{
    code_stream::CodeStream,
    token::{operator::Operator, token_value::TokenValue},
    token_collector::TokenCollector,
};

pub struct OperatorCollector;

impl TokenCollector for OperatorCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        for op in Operator::all() {
            let value = op.into();

            if code.check(value) {
                code.skip(value.len());
                return Some(TokenValue::Operator(op));
            }
        }

        None
    }
}

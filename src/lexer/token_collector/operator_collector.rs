use crate::lexer::{
    code_stream::CodeStream,
    token::{operator::Operator, token_value::TokenValue},
    token_collector::TokenCollector,
};

pub struct OperatorCollector;

impl TokenCollector for OperatorCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        for op in Operator::all() {
            let value = String::from(op);

            if code.check(value.as_str()) {
                code.skip(value.len());
                return Some(TokenValue::Operator(op));
            }
        }

        None
    }
}

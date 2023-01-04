use crate::lexer::{
    code_stream::CodeStream,
    token::token_value::{operator::Operator, TokenValue},
    token_collector::TokenCollector,
};

pub struct OperatorCollector;

impl TokenCollector for OperatorCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        for op in Operator::all() {
            let value: &str = op.into();

            if code.check(value) {
                code.skip(value.len());
                return Some(TokenValue::Operator(op));
            }
        }

        None
    }
}

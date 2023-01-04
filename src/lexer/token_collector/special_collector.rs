use crate::lexer::{
    code_stream::CodeStream,
    token::{special::Special, token_value::TokenValue},
    token_collector::TokenCollector,
};

pub struct SpecialCollector;

impl TokenCollector for SpecialCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        let value: String = code.current().to_string();

        if let Ok(special) = Special::try_from(value.as_str()) {
            code.accept();
            return Some(TokenValue::Special(special));
        }

        return None;
    }
}

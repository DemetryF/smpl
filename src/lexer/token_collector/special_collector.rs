use crate::lexer::{
    code_stream::CodeStream, token::token_value::TokenValue, token_collector::TokenCollector,
};

pub struct SpecialCollector;

impl TokenCollector for SpecialCollector {
    fn try_next(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        let value = match code_stream.current() {
            ',' => TokenValue::Comma,
            ';' => TokenValue::Semicolon,
            '(' => TokenValue::OpeningParen,
            ')' => TokenValue::ClosingParen,
            '{' => TokenValue::OpeningBrace,
            '}' => TokenValue::ClosingBrace,

            _ => return None,
        };

        code_stream.accept();

        Some(value)
    }
}

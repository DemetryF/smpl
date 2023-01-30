use parse_int::parse;

use crate::lexer::{
    code_stream::CodeStream,
    token::{Literal, TokenValue},
    token_collector::TokenCollector,
};

pub struct NumberCollector;
impl TokenCollector for NumberCollector {
    fn try_next(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        if !Self::is_digit(code_stream, 10) {
            return None;
        }

        let buffer = match code_stream.slice_from_current(2) {
            "0b" => Self::from_rad(code_stream, 2),
            "0o" => Self::from_rad(code_stream, 8),
            "0x" => Self::from_rad(code_stream, 16),
            _ => Self::decimal(code_stream),
        };

        let number = parse::<f64>(buffer.as_str()).unwrap();

        Some(TokenValue::Literal(Literal::Number(number)))
    }
}

impl NumberCollector {
    fn is_digit(code_stream: &CodeStream, rad: u32) -> bool {
        code_stream.current().is_digit(rad)
    }

    fn from_rad(code_stream: &mut CodeStream, rad: u32) -> String {
        let mut buffer = String::new();

        buffer += code_stream.skip(2); // skip prefix
        buffer += Self::num_literal(code_stream, rad).as_str();

        buffer
    }

    fn decimal(code_stream: &mut CodeStream) -> String {
        let mut buffer = Self::num_literal(code_stream, 10);

        if code_stream.check(".") {
            buffer.push(code_stream.accept());
            buffer += Self::num_literal(code_stream, 10).as_str();
        }

        buffer
    }

    fn num_literal(code_stream: &mut CodeStream, rad: u32) -> String {
        let mut buffer = String::new();

        while !code_stream.is_eof() && Self::is_digit(code_stream, rad) {
            buffer.push(code_stream.accept());
        }

        buffer
    }
}

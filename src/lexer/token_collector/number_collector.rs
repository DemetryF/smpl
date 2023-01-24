use crate::lexer::{
    code_stream::CodeStream,
    token::token_value::{Literal, TokenValue},
    token_collector::TokenCollector,
};

pub struct NumberCollector;

impl NumberCollector {
    fn is_digit(code_stream: &CodeStream) -> bool {
        code_stream.current().is_ascii_digit()
    }

    fn lex_num_from_rad(code_stream: &mut CodeStream, rad: usize) -> f64 {
        code_stream.skip(2);
        let buffer = Self::lex_num_literal(code_stream, rad);
        Self::parse_int(buffer, rad)
    }

    fn lex_dec(code_stream: &mut CodeStream) -> f64 {
        let mut buffer = Self::lex_num_literal(code_stream, 10);

        if code_stream.check(".") {
            buffer.push(code_stream.accept());
            buffer += Self::lex_num_literal(code_stream, 10).as_str();
        }

        buffer.parse().expect("NumberCollector")
    }

    fn parse_int(str: String, radix: usize) -> f64 {
        let mut result = 0;

        for i in 0..str.len() {
            let c = str[i..].chars().next().expect("");
            let n = c.to_digit(radix as u32).expect("");

            result += n * radix.pow((str.len() - i - 1) as u32) as u32;
        }

        result as f64
    }

    fn lex_num_literal(code_stream: &mut CodeStream, rad: usize) -> String {
        let mut buffer = String::new();

        while !code_stream.is_eof() && code_stream.current().is_digit(rad as u32) {
            buffer.push(code_stream.accept());
        }

        buffer
    }
}

impl TokenCollector for NumberCollector {
    fn try_next(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        if !Self::is_digit(code_stream) {
            return None;
        }

        let start = code_stream.pos.index;
        let number = match code_stream.get_code_slice(start, 2) {
            "0b" => Self::lex_num_from_rad(code_stream, 2),
            "0o" => Self::lex_num_from_rad(code_stream, 8),
            "0x" => Self::lex_num_from_rad(code_stream, 16),
            _ => Self::lex_dec(code_stream),
        };

        Some(TokenValue::Literal(Literal::Number(number)))
    }
}

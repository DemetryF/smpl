use crate::lexer::{
    code_stream::CodeStream,
    token::token_value::{Literal, TokenValue},
    token_collector::TokenCollector,
};

pub struct NumberCollector;

impl NumberCollector {
    fn is_digit(code: &CodeStream) -> bool {
        code.current().is_ascii_digit()
    }

    fn lex_num_from_rad(code: &mut CodeStream, rad: usize) -> f64 {
        code.skip(2);
        let buffer = Self::lex_num_literal(code, rad);
        Self::parse_int(buffer, rad)
    }

    fn lex_dec(code: &mut CodeStream) -> f64 {
        let mut buffer = Self::lex_num_literal(code, 10);

        if code.check(".") {
            buffer.push(code.accept());
            buffer += Self::lex_num_literal(code, 10).as_str();
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

    fn lex_num_literal(code: &mut CodeStream, rad: usize) -> String {
        let mut buffer = String::new();

        while !code.is_eof() && code.current().is_digit(rad as u32) {
            buffer.push(code.accept());
        }

        buffer
    }
}

impl TokenCollector for NumberCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        if !Self::is_digit(code) {
            return None;
        }

        let start = code.pos.index;
        let number = match code.get_code_slice(start, 2) {
            "0b" => Self::lex_num_from_rad(code, 2),
            "0o" => Self::lex_num_from_rad(code, 8),
            "0x" => Self::lex_num_from_rad(code, 16),
            _ => Self::lex_dec(code),
        };

        Some(TokenValue::Literal(Literal::Number(number)))
    }
}

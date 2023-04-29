use parse_int::parse;

use crate::lexer::{CodeStream, Literal, TokenValue};

use super::TokenCollector;

const RADIX_PREFIX_LENGTH: usize = 2;

pub struct NumberCollector;
impl TokenCollector for NumberCollector {
    fn try_collect(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        if !Self::is_digit(code_stream, 10) {
            return None;
        }
        let start = code_stream.get_index();

        match code_stream.slice_from_current(RADIX_PREFIX_LENGTH) {
            "0b" => Self::prefixed(code_stream, 2),
            "0o" => Self::prefixed(code_stream, 8),
            "0x" => Self::prefixed(code_stream, 16),
            _ => Self::common_number(code_stream),
        };

        let end = code_stream.get_index();

        let buffer = code_stream.slice(start, end);

        let number = parse(buffer).unwrap();

        Some(TokenValue::Literal(Literal::Number(number)))
    }
}

impl NumberCollector {
    pub fn is_digit(code_stream: &CodeStream, radix: u32) -> bool {
        code_stream.current().is_digit(radix)
    }

    pub fn prefixed(code_stream: &mut CodeStream, radix: u32) {
        code_stream.skip(RADIX_PREFIX_LENGTH);
        Self::number_literal(code_stream, radix);
    }

    pub fn common_number(code_stream: &mut CodeStream) {
        Self::number_literal(code_stream, 10);

        Self::fraction(code_stream);
        Self::exponential_part(code_stream);
    }

    pub fn fraction(code_stream: &mut CodeStream) {
        if code_stream.check('.') {
            code_stream.consume();

            Self::number_literal(code_stream, 10);
        }
    }

    pub fn exponential_part(code_stream: &mut CodeStream) {
        if code_stream.check('e') || code_stream.check('E') {
            code_stream.consume();

            if code_stream.check('-') || code_stream.check('+') {
                code_stream.consume();
            }

            Self::number_literal(code_stream, 10);
        }
    }

    fn number_literal(code_stream: &mut CodeStream, radix: u32) {
        while !code_stream.is_eof()
            && (Self::is_digit(code_stream, radix) || code_stream.check('_'))
        {
            code_stream.consume();
        }
    }
}

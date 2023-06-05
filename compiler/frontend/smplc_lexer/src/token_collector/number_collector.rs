use parse_int::parse;

use smplc_token::{Literal, TokenValue};

use crate::code_stream::CodeStream;

use super::TokenCollector;

const RADIX_PREFIX_LEN: usize = 2;

pub struct NumberCollector;

impl<'source> TokenCollector<'source> for NumberCollector {
    fn try_collect(&self, code_stream: &mut CodeStream<'source>) -> Option<TokenValue<'source>> {
        if !Self::is_number_start(code_stream) {
            return None;
        }

        let start = code_stream.index();

        match code_stream.slice_from_current(RADIX_PREFIX_LEN) {
            "0b" => Self::prefixed(code_stream, 2),
            "0o" => Self::prefixed(code_stream, 8),
            "0x" => Self::prefixed(code_stream, 16),

            _ => Self::unprefixed(code_stream),
        }

        let end = code_stream.index();

        let buffer = code_stream.slice(start, end);
        let value = parse::<f32>(buffer).unwrap();

        Some(TokenValue::Literal(Literal::Num(value)))
    }
}

impl NumberCollector {
    pub fn is_number_start(code_stream: &mut CodeStream) -> bool {
        code_stream.current().is_ascii_digit()
    }

    /// eats prefixed number like '0b101' or '0x42F'
    pub fn prefixed(code_stream: &mut CodeStream, radix: u32) {
        code_stream.skip_n(RADIX_PREFIX_LEN);

        Self::eat_digits(code_stream, radix);
    }

    /// eats unprefixed number like '0.42_e2'
    pub fn unprefixed(code_stream: &mut CodeStream) {
        Self::eat_digits(code_stream, 10);

        Self::fraction_part(code_stream);
        Self::exponent_part(code_stream);
    }

    pub fn eat_digits(code_stream: &mut CodeStream, radix: u32) {
        while Self::is_digit(code_stream, radix) {
            code_stream.next();
        }
    }

    pub fn fraction_part(code_stream: &mut CodeStream) {
        if code_stream.check('.') {
            code_stream.next();

            Self::eat_digits(code_stream, 10);
        }
    }

    pub fn exponent_part(code_stream: &mut CodeStream) {
        if code_stream.try_consume('e') || code_stream.try_consume('E') {
            let _ = code_stream.try_consume('+') || code_stream.try_consume('-');

            Self::eat_digits(code_stream, 10);
        }
    }

    pub fn is_digit(code_stream: &mut CodeStream, radix: u32) -> bool {
        !code_stream.is_eof() && (code_stream.current().is_digit(radix) || code_stream.check('_'))
    }
}

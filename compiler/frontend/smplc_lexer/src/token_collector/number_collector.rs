use parse_int::parse;
use smplc_ast::Literal;

use crate::{CodeStream, TokenValue};

use super::TokenCollector;

const RADIX_PREFIX_LENGTH: usize = 2;

pub struct NumberCollector;
impl TokenCollector for NumberCollector {
    fn try_collect<'source>(
        &mut self,
        code_stream: &mut CodeStream<'source>,
    ) -> Option<TokenValue<'source>> {
        if !Self::is_digit(code_stream, 10) {
            return None;
        }

        let start = code_stream.index();
        let mut is_float = false;

        match code_stream.slice_from_current(RADIX_PREFIX_LENGTH) {
            "0b" => Self::prefixed(code_stream, 2),
            "0o" => Self::prefixed(code_stream, 8),
            "0x" => Self::prefixed(code_stream, 16),
            _ => Self::common_number(code_stream, &mut is_float),
        };

        let end = code_stream.index();

        let buffer = code_stream.slice(start, end);

        match is_float {
            true => Some(TokenValue::Literal(Literal::Real(parse(buffer).unwrap()))),
            false => Some(TokenValue::Literal(Literal::Int(parse(buffer).unwrap()))),
        }
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

    pub fn common_number(code_stream: &mut CodeStream, is_float: &mut bool) {
        Self::number_literal(code_stream, 10);

        Self::fraction(code_stream, is_float);
        Self::exponential_part(code_stream, is_float);
    }

    pub fn fraction(code_stream: &mut CodeStream, is_float: &mut bool) {
        if code_stream.check('.') {
            *is_float = true;

            code_stream.next_ch();

            Self::number_literal(code_stream, 10);
        }
    }

    pub fn exponential_part(code_stream: &mut CodeStream, is_float: &mut bool) {
        if code_stream.check('e') || code_stream.check('E') {
            *is_float = true;

            code_stream.next_ch();

            if code_stream.check('-') || code_stream.check('+') {
                code_stream.next_ch();
            }

            Self::number_literal(code_stream, 10);
        }
    }

    fn number_literal(code_stream: &mut CodeStream, radix: u32) {
        while !code_stream.is_eof()
            && (Self::is_digit(code_stream, radix) || code_stream.check('_'))
        {
            code_stream.next_ch();
        }
    }
}

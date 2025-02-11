use parse_int::parse;
use smplc_ast::Literal;

use crate::{Cursor, TokenValue};

use super::TokenCollector;

const RADIX_PREFIX_LENGTH: usize = 2;

pub struct NumberCollector;
impl TokenCollector for NumberCollector {
    fn try_collect<'source>(
        &mut self,
        cursor: &mut Cursor<'source>,
    ) -> Option<TokenValue<'source>> {
        if !Self::is_digit(cursor, 10) {
            return None;
        }

        let start = cursor.index();
        let mut is_float = false;

        match cursor.slice_from_current(RADIX_PREFIX_LENGTH) {
            "0b" => Self::prefixed(cursor, 2),
            "0o" => Self::prefixed(cursor, 8),
            "0x" => Self::prefixed(cursor, 16),
            _ => Self::common_number(cursor, &mut is_float),
        };

        let end = cursor.index();

        let buffer = cursor.slice(start, end);

        match is_float {
            true => Some(TokenValue::Literal(Literal::Real(parse(buffer).unwrap()))),
            false => Some(TokenValue::Literal(Literal::Int(parse(buffer).unwrap()))),
        }
    }
}

impl NumberCollector {
    pub fn is_digit(cursor: &Cursor, radix: u32) -> bool {
        cursor.current().is_digit(radix)
    }

    pub fn prefixed(cursor: &mut Cursor, radix: u32) {
        cursor.skip(RADIX_PREFIX_LENGTH);
        Self::number_literal(cursor, radix);
    }

    pub fn common_number(cursor: &mut Cursor, is_float: &mut bool) {
        Self::number_literal(cursor, 10);

        Self::fraction(cursor, is_float);
        Self::exponential_part(cursor, is_float);
    }

    pub fn fraction(cursor: &mut Cursor, is_float: &mut bool) {
        if cursor.check('.') {
            *is_float = true;

            cursor.next_ch();

            Self::number_literal(cursor, 10);
        }
    }

    pub fn exponential_part(cursor: &mut Cursor, is_float: &mut bool) {
        if cursor.check('e') || cursor.check('E') {
            *is_float = true;

            cursor.next_ch();

            if cursor.check('-') || cursor.check('+') {
                cursor.next_ch();
            }

            Self::number_literal(cursor, 10);
        }
    }

    fn number_literal(cursor: &mut Cursor, radix: u32) {
        while !cursor.is_eof()
            && (Self::is_digit(cursor, radix) || cursor.check('_'))
        {
            cursor.next_ch();
        }
    }
}

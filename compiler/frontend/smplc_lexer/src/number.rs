use smplc_ast::Type;

use crate::cursor::Cursor;
use crate::TokenTag;

const RADIX_PREFIX_LENGTH: usize = 2;

pub fn lex_number(cursor: &mut Cursor) -> Option<TokenTag> {
    if !cursor.current().is_digit(10) {
        return None;
    }

    Some(TokenTag::Literal(
        match cursor.slice_from_current(RADIX_PREFIX_LENGTH) {
            "0b" => prefixed(cursor, 2),
            "0o" => prefixed(cursor, 8),
            "0x" => prefixed(cursor, 16),

            _ => decimal(cursor),
        },
    ))
}

pub fn prefixed(cursor: &mut Cursor, radix: u32) -> Type {
    cursor.skip(RADIX_PREFIX_LENGTH);

    literal(cursor, radix);

    Type::Int
}

pub fn decimal(cursor: &mut Cursor) -> Type {
    literal(cursor, 10);

    let has_fraction = fraction(cursor);
    let has_exponential = exponential_part(cursor);

    if has_fraction || has_exponential {
        Type::Real
    } else {
        Type::Int
    }
}

pub fn fraction(cursor: &mut Cursor) -> bool {
    if cursor.check('.') {
        cursor.next_ch();

        literal(cursor, 10);

        true
    } else {
        false
    }
}

pub fn exponential_part(cursor: &mut Cursor) -> bool {
    if cursor.check('e') || cursor.check('E') {
        cursor.next_ch();

        if cursor.check('-') || cursor.check('+') {
            cursor.next_ch();
        }

        literal(cursor, 10);

        true
    } else {
        false
    }
}

fn literal(cursor: &mut Cursor, radix: u32) {
    while !cursor.is_eof() && (cursor.current().is_digit(radix) || cursor.check('_')) {
        cursor.next_ch();
    }
}

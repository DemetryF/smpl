use parse_int::parse;
use smplc_ast::Literal;

use crate::cursor::Cursor;
use crate::TokenValue;

const RADIX_PREFIX_LENGTH: usize = 2;

pub fn lex_number<'source>(cursor: &mut Cursor<'source>) -> Option<TokenValue<'source>> {
    if !cursor.current().is_digit(10) {
        return None;
    }

    let start = cursor.index();
    let mut is_float = false;

    match cursor.slice_from_current(RADIX_PREFIX_LENGTH) {
        "0b" => prefixed(cursor, 2),
        "0o" => prefixed(cursor, 8),
        "0x" => prefixed(cursor, 16),

        _ => decimal(cursor, &mut is_float),
    };

    let end = cursor.index();

    let buffer = cursor.slice(start, end);

    match is_float {
        true => Some(TokenValue::Literal(Literal::Real(parse(buffer).unwrap()))),
        false => Some(TokenValue::Literal(Literal::Int(parse(buffer).unwrap()))),
    }
}

pub fn prefixed(cursor: &mut Cursor, radix: u32) {
    cursor.skip(RADIX_PREFIX_LENGTH);
    literal(cursor, radix);
}

pub fn decimal(cursor: &mut Cursor, is_float: &mut bool) {
    literal(cursor, 10);

    fraction(cursor, is_float);
    exponential_part(cursor, is_float);
}

pub fn fraction(cursor: &mut Cursor, is_float: &mut bool) {
    if cursor.check('.') {
        *is_float = true;

        cursor.next_ch();

        literal(cursor, 10);
    }
}

pub fn exponential_part(cursor: &mut Cursor, is_float: &mut bool) {
    if cursor.check('e') || cursor.check('E') {
        *is_float = true;

        cursor.next_ch();

        if cursor.check('-') || cursor.check('+') {
            cursor.next_ch();
        }

        literal(cursor, 10);
    }
}

fn literal(cursor: &mut Cursor, radix: u32) {
    while !cursor.is_eof() && (cursor.current().is_digit(radix) || cursor.check('_')) {
        cursor.next_ch();
    }
}

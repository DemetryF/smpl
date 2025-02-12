use crate::cursor::Cursor;
use crate::TokenTag;

pub fn two_char_specials(cursor: &mut Cursor) -> Option<TokenTag> {
    match cursor.slice_from_current(2) {
        ">=" => Some(TokenTag::Ge),
        "<=" => Some(TokenTag::Le),
        "!=" => Some(TokenTag::Ne),
        "==" => Some(TokenTag::Eq),
        "->" => Some(TokenTag::Arrow),

        _ => None,
    }
    .inspect(|_| cursor.skip(2))
}

pub fn one_char_specials(cursor: &mut Cursor) -> Option<TokenTag> {
    match cursor.current() {
        ';' => Some(TokenTag::Semicolon),
        ',' => Some(TokenTag::Comma),
        ':' => Some(TokenTag::Colon),
        '{' => Some(TokenTag::LBrace),
        '}' => Some(TokenTag::RBrace),
        '(' => Some(TokenTag::LParen),
        ')' => Some(TokenTag::RParen),
        '=' => Some(TokenTag::Assign),
        '|' => Some(TokenTag::Or),
        '&' => Some(TokenTag::And),
        '>' => Some(TokenTag::Gt),
        '<' => Some(TokenTag::Lt),
        '+' => Some(TokenTag::Plus),
        '-' => Some(TokenTag::Minus),
        '*' => Some(TokenTag::Star),
        '/' => Some(TokenTag::Slash),
        '!' => Some(TokenTag::Not),

        _ => None,
    }
    .inspect(|_| cursor.skip(1))
}

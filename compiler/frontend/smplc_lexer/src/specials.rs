use crate::cursor::Cursor;
use crate::TokenValue;

pub fn two_char_specials<'source>(cursor: &mut Cursor<'source>) -> Option<TokenValue<'source>> {
    match cursor.slice_from_current(2) {
        ">=" => Some(TokenValue::Ge),
        "<=" => Some(TokenValue::Le),
        "!=" => Some(TokenValue::Ne),
        "==" => Some(TokenValue::Eq),
        "->" => Some(TokenValue::Arrow),

        _ => None,
    }
    .inspect(|_| cursor.skip(2))
}

pub fn one_char_specials<'source>(cursor: &mut Cursor<'source>) -> Option<TokenValue<'source>> {
    match cursor.current() {
        ';' => Some(TokenValue::Semicolon),
        ',' => Some(TokenValue::Comma),
        ':' => Some(TokenValue::Colon),
        '{' => Some(TokenValue::LBrace),
        '}' => Some(TokenValue::RBrace),
        '(' => Some(TokenValue::LParen),
        ')' => Some(TokenValue::RParen),
        '=' => Some(TokenValue::Assign),
        '|' => Some(TokenValue::Or),
        '&' => Some(TokenValue::And),
        '>' => Some(TokenValue::Gt),
        '<' => Some(TokenValue::Lt),
        '+' => Some(TokenValue::Plus),
        '-' => Some(TokenValue::Minus),
        '*' => Some(TokenValue::Star),
        '/' => Some(TokenValue::Slash),
        '!' => Some(TokenValue::Not),

        _ => None,
    }
    .inspect(|_| cursor.skip(1))
}

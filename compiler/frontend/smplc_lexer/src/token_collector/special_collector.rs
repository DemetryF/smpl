use crate::{Cursor, TokenValue};

use super::TokenCollector;

pub struct SpecialCollector;
impl TokenCollector for SpecialCollector {
    fn try_collect<'source>(
        &mut self,
        cursor: &mut Cursor<'source>,
    ) -> Option<TokenValue<'source>> {
        Self::double(cursor).or(Self::single(cursor))
    }
}

impl SpecialCollector {
    pub fn double<'source>(cursor: &mut Cursor<'source>) -> Option<TokenValue<'source>> {
        let value = match cursor.slice_from_current(2) {
            ">=" => TokenValue::Ge,
            "<=" => TokenValue::Le,
            "!=" => TokenValue::Ne,
            "==" => TokenValue::Eq,
            "->" => TokenValue::Arrow,

            _ => return None,
        };

        cursor.skip(2);

        Some(value)
    }

    pub fn single<'source>(cursor: &mut Cursor<'source>) -> Option<TokenValue<'source>> {
        let value = match cursor.current() {
            ';' => TokenValue::Semicolon,
            ',' => TokenValue::Comma,
            ':' => TokenValue::Colon,
            '{' => TokenValue::LBrace,
            '}' => TokenValue::RBrace,
            '(' => TokenValue::LParen,
            ')' => TokenValue::RParen,
            '=' => TokenValue::Assign,
            '|' => TokenValue::Or,
            '&' => TokenValue::And,
            '>' => TokenValue::Gt,
            '<' => TokenValue::Lt,
            '+' => TokenValue::Plus,
            '-' => TokenValue::Minus,
            '*' => TokenValue::Star,
            '/' => TokenValue::Slash,
            '!' => TokenValue::Not,

            _ => return None,
        };

        cursor.next_ch();

        Some(value)
    }
}

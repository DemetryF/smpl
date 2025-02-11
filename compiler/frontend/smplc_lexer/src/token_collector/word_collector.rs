use smplc_ast::{Literal, Type};

use crate::{Cursor, TokenValue};

use super::TokenCollector;

pub struct WordCollector;
impl TokenCollector for WordCollector {
    fn try_collect<'source>(
        &mut self,
        cursor: &mut Cursor<'source>,
    ) -> Option<TokenValue<'source>> {
        if !Self::is_word_char(cursor) {
            return None;
        }

        let buffer = Self::word_literal(cursor);

        Some(match buffer {
            "let" => TokenValue::Let,
            "else" => TokenValue::Else,
            "fn" => TokenValue::Fn,
            "if" => TokenValue::If,
            "return" => TokenValue::Return,
            "while" => TokenValue::While,
            "const" => TokenValue::Const,

            "continue" => TokenValue::Continue,
            "break" => TokenValue::Break,

            "true" => TokenValue::Literal(Literal::Bool(true)),
            "false" => TokenValue::Literal(Literal::Bool(false)),

            "real" => TokenValue::Type(Type::Real),
            "int" => TokenValue::Type(Type::Int),
            "bool" => TokenValue::Type(Type::Bool),

            id => TokenValue::Id(id),
        })
    }
}

impl WordCollector {
    fn is_word_char(cursor: &Cursor) -> bool {
        cursor.current().is_ascii_alphabetic() || cursor.check('$') || cursor.check('_')
    }

    fn word_literal<'source>(cursor: &mut Cursor<'source>) -> &'source str {
        let start = cursor.index();

        while !cursor.is_eof() && (Self::is_word_char(cursor) || cursor.current().is_alphanumeric())
        {
            cursor.next_ch();
        }

        let end = cursor.index();

        cursor.slice(start, end)
    }
}

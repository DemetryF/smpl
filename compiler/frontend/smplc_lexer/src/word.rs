use smplc_ast::Literal;
use smplc_ast::Type;

use crate::cursor::Cursor;
use crate::TokenValue;

pub fn lex_word<'source>(cursor: &mut Cursor<'source>) -> Option<TokenValue<'source>> {
    if !(cursor.current().is_alphabetic() || cursor.current() == '_' || cursor.current() == '$') {
        return None;
    }

    let buffer = word_literal(cursor);

    let value = match buffer {
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
    };

    Some(value)
}

fn word_literal<'source>(cursor: &mut Cursor<'source>) -> &'source str {
    let start = cursor.index();

    while !cursor.is_eof()
        && (cursor.current().is_alphanumeric()
            || cursor.current() == '_'
            || cursor.current() == '$')
    {
        cursor.next_ch();
    }

    let end = cursor.index();

    cursor.slice(start, end)
}

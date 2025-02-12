use smplc_ast::Type;

use crate::cursor::Cursor;
use crate::TokenTag;

pub fn lex_word(cursor: &mut Cursor) -> Option<TokenTag> {
    if !(cursor.current().is_alphabetic() || cursor.current() == '_' || cursor.current() == '$') {
        return None;
    }

    let buffer = word_literal(cursor);

    let value = match buffer {
        "let" => TokenTag::Let,
        "else" => TokenTag::Else,
        "fn" => TokenTag::Fn,
        "if" => TokenTag::If,
        "return" => TokenTag::Return,
        "while" => TokenTag::While,
        "const" => TokenTag::Const,

        "continue" => TokenTag::Continue,
        "break" => TokenTag::Break,

        "true" => TokenTag::Literal(Type::Bool),
        "false" => TokenTag::Literal(Type::Bool),

        "real" => TokenTag::Type(Type::Real),
        "int" => TokenTag::Type(Type::Int),
        "bool" => TokenTag::Type(Type::Bool),

        _ => TokenTag::Id,
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

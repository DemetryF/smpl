use smplc_ast::LiteralType;

use crate::{cursor::Cursor, TokenTag};

pub fn lex_word(cursor: &mut Cursor) -> Option<TokenTag> {
    if !is_word_start(cursor.current()) {
        return None;
    }

    Some(match word_literal(cursor) {
        "let" => TokenTag::Let,
        "else" => TokenTag::Else,
        "fn" => TokenTag::Fn,
        "if" => TokenTag::If,
        "return" => TokenTag::Return,
        "while" => TokenTag::While,
        "const" => TokenTag::Const,

        "continue" => TokenTag::Continue,
        "break" => TokenTag::Break,

        "true" => TokenTag::Literal(LiteralType::Bool),
        "false" => TokenTag::Literal(LiteralType::Bool),

        _ => TokenTag::Id,
    })
}

fn is_word_start(char: char) -> bool {
    char.is_alphabetic() || matches!(char, '_' | '$')
}

fn word_literal<'source>(cursor: &mut Cursor<'source>) -> &'source str {
    let start = cursor.index();

    while !cursor.is_eof()
        && (cursor.current().is_alphanumeric() || matches!(cursor.current(), '_' | '$'))
    {
        cursor.next_ch();
    }

    let end = cursor.index();

    cursor.slice(start, end)
}

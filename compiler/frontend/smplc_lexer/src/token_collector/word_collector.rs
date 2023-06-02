use smplc_token::{Literal, TokenValue};

use crate::code_stream::CodeStream;

use super::TokenCollector;

pub struct WordCollector;
impl TokenCollector for WordCollector {
    fn try_collect(&self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        if !Self::is_word_start(code_stream) {
            return None;
        }

        let buffer = Self::word_literal(code_stream);

        Some(match buffer {
            "let" => TokenValue::Let,
            "else" => TokenValue::Else,
            "fn" => TokenValue::Fn,
            "if" => TokenValue::If,
            "return" => TokenValue::Return,
            "while" => TokenValue::While,

            "true" => TokenValue::Literal(Literal::Bool(true)),
            "false" => TokenValue::Literal(Literal::Bool(false)),

            id => TokenValue::Ident(String::from(id)),
        })
    }
}

impl WordCollector {
    fn is_word_start(code_stream: &CodeStream) -> bool {
        code_stream.current().is_ascii_alphabetic()
            || code_stream.check('$')
            || code_stream.check('_')
    }

    fn is_word_char(code_stream: &mut CodeStream) -> bool {
        !code_stream.is_eof()
            && (Self::is_word_start(code_stream) || code_stream.current().is_alphanumeric())
    }

    fn word_literal<'source>(code_stream: &'source mut CodeStream) -> &'source str {
        let start = code_stream.index();

        while Self::is_word_char(code_stream) {
            code_stream.next();
        }

        let end = code_stream.index();

        code_stream.slice(start, end)
    }
}

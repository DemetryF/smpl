use crate::code_stream::CodeStream;
use crate::token::{Literal, TokenValue};
use crate::TokenCollector;

pub struct WordCollector;

impl<'source> TokenCollector<'source> for WordCollector {
    fn try_collect(&self, code_stream: &mut CodeStream<'source>) -> Option<TokenValue<'source>> {
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

            "break" => TokenValue::Break,
            "continue" => TokenValue::Continue,

            "true" => TokenValue::Literal(Literal::Bool(true)),
            "false" => TokenValue::Literal(Literal::Bool(false)),

            id => TokenValue::Ident(id),
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

    fn word_literal<'source>(code_stream: &mut CodeStream<'source>) -> &'source str {
        let start = code_stream.index();

        while Self::is_word_char(code_stream) {
            code_stream.next();
        }

        let end = code_stream.index();

        code_stream.slice(start, end)
    }
}

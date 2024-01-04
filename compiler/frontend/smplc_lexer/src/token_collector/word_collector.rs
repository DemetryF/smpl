use smplc_ast::Literal;

use crate::{CodeStream, TokenValue};

use super::TokenCollector;

pub struct WordCollector;
impl TokenCollector for WordCollector {
    fn try_collect<'source>(
        &mut self,
        code_stream: &mut CodeStream<'source>,
    ) -> Option<TokenValue<'source>> {
        if !Self::is_word_char(code_stream) {
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
            "const" => TokenValue::Const,

            "continue" => TokenValue::Continue,
            "break" => TokenValue::Break,

            "true" => TokenValue::Literal(Literal::Bool(true)),
            "false" => TokenValue::Literal(Literal::Bool(false)),

            id => TokenValue::Id(id),
        })
    }
}

impl WordCollector {
    fn is_word_char(code_stream: &CodeStream) -> bool {
        code_stream.current().is_ascii_alphabetic()
            || code_stream.check('$')
            || code_stream.check('_')
    }

    fn word_literal<'source>(code_stream: &mut CodeStream<'source>) -> &'source str {
        let start = code_stream.index();

        while !code_stream.is_eof()
            && (Self::is_word_char(code_stream) || code_stream.current().is_alphanumeric())
        {
            code_stream.next_ch();
        }

        let end = code_stream.index();

        code_stream.slice(start, end)
    }
}

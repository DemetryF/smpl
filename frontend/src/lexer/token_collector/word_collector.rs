use crate::lexer::{CodeStream, Literal, TokenCollector, TokenValue};

pub struct WordCollector;
impl TokenCollector for WordCollector {
    fn try_collect(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
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

            "true" => TokenValue::Literal(Literal::Bool(true)),
            "false" => TokenValue::Literal(Literal::Bool(false)),

            id => TokenValue::Id(String::from(id)),
        })
    }
}

impl WordCollector {
    fn is_word_char(code_stream: &CodeStream) -> bool {
        code_stream.current().is_ascii_alphabetic()
            || code_stream.check('$')
            || code_stream.check('_')
    }

    fn word_literal<'code>(code_stream: &'code mut CodeStream) -> &'code str {
        let start = code_stream.get_index();

        while !code_stream.is_eof()
            && (Self::is_word_char(code_stream) || code_stream.current().is_alphanumeric())
        {
            code_stream.consume();
        }

        let end = code_stream.get_index();

        code_stream.slice(start, end)
    }
}

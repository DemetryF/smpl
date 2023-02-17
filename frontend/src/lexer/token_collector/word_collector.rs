use crate::{
    lexer::{CodeStream, TokenCollector},
    token::{Literal, TokenValue},
};

pub struct WordCollector;
impl TokenCollector for WordCollector {
    fn try_next(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        if !Self::is_word_char(code_stream) {
            return None;
        }

        let start = code_stream.get_pos().index;
        let len = Self::lex_word_literal(code_stream);

        Some(match code_stream.slice(start, len) {
            "let" => TokenValue::Define,
            "else" => TokenValue::Else,
            "fn" => TokenValue::Function,
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
            || code_stream.check("$")
            || code_stream.check("_")
    }

    fn lex_word_literal(code_stream: &mut CodeStream) -> usize {
        let mut len = 0;

        while !code_stream.is_eof()
            && (Self::is_word_char(code_stream) || code_stream.current().is_alphanumeric())
        {
            code_stream.accept();
            len += 1;
        }

        len
    }
}

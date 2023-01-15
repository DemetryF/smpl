use crate::lexer::{
    code_stream::CodeStream,
    token::token_value::{Literal, TokenValue},
    token_collector::TokenCollector,
};

pub struct WordCollector;

impl WordCollector {
    fn is_word_char(code: &CodeStream) -> bool {
        code.current().is_ascii_alphanumeric()
    }

    fn lex_word_literal(code: &mut CodeStream) -> usize {
        let mut len = 0;

        while !code.is_eof() && Self::is_word_char(code) {
            code.accept();
            len += 1;
        }

        len
    }
}

impl TokenCollector for WordCollector {
    fn try_next<'code>(&mut self, code: &mut CodeStream<'code>) -> Option<TokenValue<'code>> {
        if !code.current().is_alphabetic() {
            return None;
        }

        let start = code.pos.index;
        let len = Self::lex_word_literal(code);

        Some(match code.get_code_slice(start, len) {
            "define" => TokenValue::Define,
            "else" => TokenValue::Else,
            "function" => TokenValue::Function,
            "if" => TokenValue::If,
            "return" => TokenValue::Return,
            "while" => TokenValue::While,

            "true" => TokenValue::Literal(Literal::Bool(true)),
            "false" => TokenValue::Literal(Literal::Bool(false)),

            id => TokenValue::Id(id),
        })
    }
}

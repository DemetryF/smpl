use crate::lexer::{
    code_stream::CodeStream,
    token::{literal::Literal, token_value::TokenValue},
    token_collector::TokenCollector,
};

pub struct WordCollector;

impl WordCollector {
    fn is_word_char(code: &CodeStream) -> bool {
        code.current().is_alphabetic() || code.current().is_ascii_digit()
    }

    fn lex_word_literal(code: &mut CodeStream) -> String {
        let mut str: String = code.accept().to_string();

        while !code.is_eof() && Self::is_word_char(code) {
            str += code.accept().to_string().as_mut();
        }

        str
    }
}

impl TokenCollector for WordCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        if !code.current().is_alphabetic() {
            return None;
        }

        let value: String = Self::lex_word_literal(code);

        Some(match value.as_str() {
            "define" => TokenValue::Define,
            "else" => TokenValue::Else,
            "function" => TokenValue::Function,
            "if" => TokenValue::If,
            "return" => TokenValue::Return,
            "while" => TokenValue::While,

            "true" => TokenValue::Literal(Literal::Bool(true)),
            "false" => TokenValue::Literal(Literal::Bool(false)),

            _ => TokenValue::Id(value),
        })
    }
}

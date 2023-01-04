use crate::lexer::{
    code_stream::CodeStream,
    token::token_value::{keyword::Keyword, TokenValue},
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

        if let Ok(keyword) = Keyword::try_from(value.as_str()) {
            return Some(TokenValue::Keyword(keyword));
        }

        Some(match value.as_str() {
            "true" => TokenValue::Bool(true),
            "false" => TokenValue::Bool(false),
            _ => TokenValue::Id(value),
        })
    }
}

use crate::lexer::{
    code_stream::CodeStream,
    token::{literal::Literal, token_value::TokenValue},
    token_collector::TokenCollector,
};

pub struct NumberCollector;

impl NumberCollector {
    fn is_digit(code: &CodeStream) -> bool {
        code.current().is_ascii_digit()
    }

    fn lex_number_literal(code: &mut CodeStream) -> String {
        let mut str = String::new();

        while !code.is_eof() && Self::is_digit(code) {
            str.push(code.accept());
        }

        str
    }
}

impl TokenCollector for NumberCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        if !(Self::is_digit(code) || code.check(".")) {
            return None;
        }

        let mut source = Self::lex_number_literal(code);

        if code.check(".") {
            source.push(code.accept());
            source.push_str(Self::lex_number_literal(code).as_mut());
        }

        let number = source.parse().expect("NumberCollector");

        Some(TokenValue::Literal(Literal::Number(source, number)))
    }
}

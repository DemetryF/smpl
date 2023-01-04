use crate::lexer::{
    code_stream::CodeStream, token::token_value::TokenValue, token_collector::TokenCollector,
};

pub struct NumberCollector;

impl NumberCollector {
    fn is_digit(code: &CodeStream) -> bool {
        code.current().is_ascii_digit()
    }

    fn lex_number_literal(code: &mut CodeStream) -> String {
        let mut str: String = "".into();

        while !code.is_eof() && Self::is_digit(code) {
            str += code.accept().to_string().as_mut()
        }

        str
    }
}

impl TokenCollector for NumberCollector {
    fn try_next(&mut self, code: &mut CodeStream) -> Option<TokenValue> {
        if !Self::is_digit(code) && !code.check(".") {
            return None;
        }

        let mut value: String = Self::lex_number_literal(code);

        if code.check(".") {
            value += code.accept().to_string().as_mut();
            value += Self::lex_number_literal(code).as_mut();
        }

        return Some(TokenValue::Number(value));
    }
}

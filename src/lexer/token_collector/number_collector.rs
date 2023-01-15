use crate::lexer::{
    code_stream::CodeStream,
    token::token_value::{Literal, TokenValue},
    token_collector::TokenCollector,
};

pub struct NumberCollector;

impl NumberCollector {
    fn is_digit(code: &CodeStream) -> bool {
        code.current().is_ascii_digit()
    }

    fn lex_number_literal(code: &mut CodeStream) -> usize {
        let mut len = 0;

        while !code.is_eof() && Self::is_digit(code) {
            code.accept();
            len += 1;
        }

        len
    }
}

impl TokenCollector for NumberCollector {
    fn try_next<'code>(&mut self, code: &mut CodeStream<'code>) -> Option<TokenValue<'code>> {
        if !Self::is_digit(code) {
            return None;
        }

        let start = code.pos.index;
        let mut len = Self::lex_number_literal(code);

        if code.check(".") {
            code.accept();
            len += 1 + Self::lex_number_literal(code);
        }

        let source = code.get_code_slice(start, len);
        let number = source.parse().expect("NumberCollector");

        Some(TokenValue::Literal(Literal::Number(number)))
    }
}

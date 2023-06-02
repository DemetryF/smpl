use smplc_token::TokenValue;

use crate::code_stream::CodeStream;

use super::TokenCollector;

pub struct SpecialCollector;
impl TokenCollector for SpecialCollector {
    fn try_collect(&self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        Self::double(code_stream).or(Self::single(code_stream))
    }
}

impl SpecialCollector {
    pub fn double(code_stream: &mut CodeStream) -> Option<TokenValue> {
        let value = {
            match code_stream.slice_from_current(2) {
                ">=" => TokenValue::Ge,
                "<=" => TokenValue::Le,
                "!=" => TokenValue::Ne,
                "==" => TokenValue::Eq,

                _ => return None,
            }
        };

        code_stream.skip_n(2);

        Some(value)
    }

    pub fn single(code_stream: &mut CodeStream) -> Option<TokenValue> {
        let value = {
            match code_stream.current() {
                ';' => TokenValue::Semicolon,
                ',' => TokenValue::Comma,
                '{' => TokenValue::LBrace,
                '}' => TokenValue::RBrace,
                '(' => TokenValue::LParen,
                ')' => TokenValue::RParen,
                '=' => TokenValue::Assign,
                '|' => TokenValue::Or,
                '&' => TokenValue::And,
                '>' => TokenValue::Gt,
                '<' => TokenValue::Lt,
                '+' => TokenValue::Plus,
                '-' => TokenValue::Minus,
                '*' => TokenValue::Asterisk,
                '/' => TokenValue::Slash,
                '!' => TokenValue::Not,

                _ => return None,
            }
        };

        code_stream.skip_n(1);

        Some(value)
    }
}

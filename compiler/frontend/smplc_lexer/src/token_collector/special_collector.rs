use smplc_token::TokenValue;

use crate::code_stream::CodeStream;

use super::TokenCollector;

pub struct SpecialCollector;
impl<'source> TokenCollector<'source> for SpecialCollector {
    fn try_collect(&self, code_stream: &mut CodeStream<'source>) -> Option<TokenValue<'source>> {
        Self::double(code_stream).or(Self::single(code_stream))
    }
}

impl SpecialCollector {
    pub fn double<'source>(code_stream: &mut CodeStream<'source>) -> Option<TokenValue<'source>> {
        let value = {
            match code_stream.slice_from_current(2) {
                "+=" => TokenValue::AddAssign,
                "-=" => TokenValue::SubAssign,
                "*=" => TokenValue::MulAssign,
                "/=" => TokenValue::DivAssign,

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

    pub fn single<'source>(code_stream: &mut CodeStream<'source>) -> Option<TokenValue<'source>> {
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

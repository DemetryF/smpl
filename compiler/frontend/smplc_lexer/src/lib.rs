mod cursor;
mod error;
mod number;
mod skip;
mod specials;
mod token;
mod word;

#[cfg(test)]
mod tests;

pub use error::LexError;
pub use token::{Token, TokenValue};

use cursor::Cursor;
use number::lex_number;
use skip::skip;
use smplc_ast::{Pos, Span};
use specials::{one_char_specials, two_char_specials};
use word::lex_word;

pub struct Lexer<'source> {
    cursor: Cursor<'source>,
    ended: bool,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            cursor: Cursor::new(source),
            ended: false,
        }
    }

    pub fn next_token(&mut self) -> Result<Token<'source>, LexError> {
        skip(&mut self.cursor);

        let start = self.cursor.get_pos();

        if self.cursor.is_eof() {
            let eof_token = Token {
                value: TokenValue::EOF,
                span: Span::with_len(start, 1),
            };

            self.ended = true;

            return Ok(eof_token);
        }

        if let Some(value) = lex(&mut self.cursor) {
            let end = self.cursor.index();

            Ok(Token {
                value,
                span: Span::with_end(start, end),
            })
        } else {
            Err(self.unexpected_char(start))
        }
    }

    fn unexpected_char(&mut self, pos: Pos) -> LexError {
        LexError {
            char: self.cursor.next_ch(),
            pos,
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<Token<'source>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }

        Some(self.next_token())
    }
}

pub fn lex<'source>(cursor: &mut Cursor<'source>) -> Option<TokenValue<'source>> {
    lex_number(cursor)
        .or_else(|| lex_word(cursor))
        .or_else(|| two_char_specials(cursor))
        .or_else(|| one_char_specials(cursor))
}

use smplc_ast::{Pos, Span};

use crate::comment_handler::CommentsHandler;
use crate::cursor::Cursor;
use crate::token_collector::*;
use crate::{LexError, Token, TokenValue};

pub struct Lexer<'source> {
    cursor: Cursor<'source>,
    collectors: Vec<Box<dyn TokenCollector>>,

    ended: bool,
}

impl<'source> Lexer<'source> {
    pub fn new(code: &'source str) -> Self {
        Self {
            cursor: Cursor::new(code),
            collectors: vec![
                Box::new(NumberCollector),
                Box::new(SpecialCollector),
                Box::new(WordCollector),
            ],
            ended: false,
        }
    }

    pub fn next_token(&mut self) -> Result<Token<'source>, LexError> {
        CommentsHandler::skip(&mut self.cursor);

        let start = self.cursor.get_pos();

        if self.cursor.is_eof() {
            let eof_token = Token {
                value: TokenValue::EOF,
                span: Span::with_len(start, 1),
            };

            self.ended = true;

            return Ok(eof_token);
        }

        for collector in self.collectors.iter_mut() {
            if let Some(value) = collector.try_collect(&mut self.cursor) {
                let span = Span::with_end(start, self.cursor.index());

                return Ok(Token { value, span });
            }
        }

        Err(self.unexpected_char(start))
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

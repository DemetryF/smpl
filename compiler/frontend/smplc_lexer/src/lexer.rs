use smplc_ast::Pos;

use crate::code_stream::CodeStream;
use crate::comment_handler::CommentsHandler;
use crate::token_collector::*;
use crate::{LexError, Token, TokenValue};

pub struct Lexer<'source> {
    code_stream: CodeStream<'source>,
    collectors: Vec<Box<dyn TokenCollector>>,
    ended: bool,
}

impl<'source> Lexer<'source> {
    pub fn new(code: &'source str) -> Self {
        Self {
            code_stream: CodeStream::new(code),
            collectors: vec![
                Box::new(NumberCollector),
                Box::new(SpecialCollector),
                Box::new(WordCollector),
            ],
            ended: false,
        }
    }

    pub fn next_token(&mut self) -> Result<Token<'source>, LexError> {
        CommentsHandler::skip(&mut self.code_stream);

        let pos = self.code_stream.get_pos();

        if self.code_stream.is_eof() {
            let eof_token = Token {
                value: TokenValue::EOF,
                pos,
            };

            self.ended = true;

            return Ok(eof_token);
        }

        for collector in self.collectors.iter_mut() {
            if let Some(value) = collector.try_collect(&mut self.code_stream) {
                let new_token = Token { value, pos };

                return Ok(new_token);
            }
        }

        Err(self.unexpected_char(pos))
    }

    fn unexpected_char(&mut self, pos: Pos) -> LexError {
        LexError {
            char: self.code_stream.next_ch(),
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

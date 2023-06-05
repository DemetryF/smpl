mod code_stream;
mod comments_handler;
mod lex_error;
mod token_collector;

use comments_handler::CommentsHandler;
use lex_error::LexError;
use smplc_token::{Pos, Token, TokenValue};

use code_stream::CodeStream;
use token_collector::{NumberCollector, SpecialCollector, TokenCollector, WordCollector};

pub struct Lexer<'source> {
    code_stream: CodeStream<'source>,
    collectors: Vec<Box<dyn TokenCollector<'source>>>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            code_stream: CodeStream::new(source),
            collectors: vec![
                Box::new(NumberCollector),
                Box::new(WordCollector),
                Box::new(SpecialCollector),
            ],
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        CommentsHandler::skip(&mut self.code_stream);

        let pos = self.code_stream.pos();

        if self.code_stream.is_eof() {
            let eof_token = Token {
                value: TokenValue::EOF,
                pos,
            };

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
            char: self.code_stream.next(),
            pos,
        }
    }
}

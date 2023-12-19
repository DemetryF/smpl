use smplc_ast::Pos;

use crate::{
    code_stream::CodeStream,
    comment_handler::CommentsHandler,
    token_collector::{NumberCollector, SpecialCollector, TokenCollector, WordCollector},
    LexError, Token, TokenValue,
};

pub struct Lexer<'code> {
    code_stream: CodeStream<'code>,
    collectors: Vec<Box<dyn TokenCollector>>,
    ended: bool,
}

impl<'code> Lexer<'code> {
    pub fn new(code: &'code str) -> Self {
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

    pub fn next_token(&mut self) -> Result<Token, LexError> {
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
            char: self.code_stream.consume(),
            pos,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }

        Some(self.next_token())
    }
}

mod code_stream;
mod comment_handler;
mod pos;
mod token;
mod token_collector;

#[cfg(test)]
mod tests;

pub use self::{
    pos::Pos,
    token::{Literal, Token, TokenValue},
};

use crate::error::{Error, ErrorKind};

use self::{
    code_stream::CodeStream,
    comment_handler::CommentsHandler,
    token_collector::{NumberCollector, SpecialCollector, TokenCollector, WordCollector},
};

pub struct Lexer<'code> {
    code_stream: CodeStream<'code>,
    collectors: Vec<Box<dyn TokenCollector>>,
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
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        CommentsHandler::skip(&mut self.code_stream);

        let pos = self.code_stream.get_pos();

        if self.code_stream.is_eof() {
            let eof_token = Token {
                value: TokenValue::EOF,
                pos,
            };

            return Ok(eof_token);
        }

        for collector in self.collectors.iter_mut() {
            if let Some(token_value) = collector.try_collect(&mut self.code_stream) {
                let new_token = Token::new(token_value, pos);

                return Ok(new_token);
            }
        }

        Err(self.unexpected_char(pos))
    }

    fn unexpected_char(&mut self, pos: Pos) -> Error {
        Error::new(ErrorKind::UnexpectedChar(self.code_stream.consume()), pos)
    }
}

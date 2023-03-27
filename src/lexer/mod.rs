use crate::error::{Error, ErrorKind};

use self::{
    code_stream::CodeStream,
    comment_handler::CommentsHandler,
    pos::Pos,
    token::{Literal, Token, TokenValue},
    token_collector::{NumberCollector, SpecialCollector, TokenCollector, WordCollector},
};

mod code_stream;
mod comment_handler;
pub mod pos;
#[cfg(test)]
mod tests;
pub mod token;
mod token_collector;

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
            return Ok(Token::new(TokenValue::EOF, pos));
        }

        for collector in self.collectors.iter_mut() {
            if let Some(token_value) = collector.try_collect(&mut self.code_stream) {
                return Ok(Token::new(token_value, pos));
            }
        }

        Err(self.unexpected_char(pos))
    }

    fn unexpected_char(&mut self, pos: Pos) -> Error {
        Error::new(ErrorKind::UnexpectedChar(self.code_stream.consume()), pos)
    }
}

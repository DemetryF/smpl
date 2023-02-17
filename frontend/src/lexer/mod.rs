use self::{code_stream::CodeStream, comments_handler::CommentsHandler, token_collector::*};

use crate::{
    error::*,
    token::{Pos, Token, TokenValue},
};

mod code_stream;
mod comments_handler;
mod token_collector;

pub struct Lexer<'code> {
    pub collectors: Vec<Box<dyn TokenCollector>>,
    pub code_stream: CodeStream<'code>,
}

impl<'code> Lexer<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            code_stream: CodeStream::new(code),
            collectors: vec![
                Box::new(NumberCollector),
                Box::new(OperatorCollector),
                Box::new(SpecialCollector),
                Box::new(WordCollector),
            ],
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        CommentsHandler::skip(&mut self.code_stream);

        let pos = self.code_stream.get_pos();

        if self.code_stream.is_eof() {
            return Ok(Token::new(TokenValue::Eof, pos));
        }

        for collector in self.collectors.iter_mut() {
            if let Some(token_value) = collector.try_next(&mut self.code_stream) {
                return Ok(Token::new(token_value, pos));
            }
        }

        Err(self.unexpected_char(pos))
    }

    fn unexpected_char(&mut self, pos: Pos) -> Error {
        Error::new(ErrorKind::UnexpectedChar(self.code_stream.accept()), pos)
    }
}

use self::{
    code_stream::CodeStream,
    comments_handler::CommentsHandler,
    pos::Pos,
    token::{token_value::TokenValue, Token},
    token_collector::{
        number_collector::NumberCollector, operator_collector::OperatorCollector,
        special_collector::SpecialCollector, word_collector::WordCollector, TokenCollector,
    },
    unexpected_token::UnexpectedToken,
};

mod code_stream;
mod comments_handler;
pub mod pos;
pub mod token;
mod token_collector;
pub mod unexpected_token;

pub struct Lexer<'code> {
    pub collectors: Vec<Box<dyn TokenCollector>>,
    pub code: CodeStream<'code>,
}

impl<'code> Lexer<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            code: CodeStream::new(code),
            collectors: vec![
                Box::new(NumberCollector),
                Box::new(OperatorCollector),
                Box::new(SpecialCollector),
                Box::new(WordCollector),
            ],
        }
    }

    pub fn next_token(&mut self) -> Result<Token, UnexpectedToken> {
        CommentsHandler::skip(&mut self.code);

        let pos = self.code.get_pos();

        if self.code.is_eof() {
            return Ok(Token::new(TokenValue::Eof, pos));
        }

        for collector in self.collectors.iter_mut() {
            if let Some(token_value) = collector.try_next(&mut self.code) {
                return Ok(Token::new(token_value, pos));
            }
        }

        self.fail(pos)
    }

    fn fail(&mut self, pos: Pos) -> Result<Token, UnexpectedToken> {
        Err(UnexpectedToken {
            value: self.code.accept().to_string(),
            pos,
        })
    }
}

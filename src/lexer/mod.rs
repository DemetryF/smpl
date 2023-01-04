use self::{
    code_stream::CodeStream,
    comments::Comments,
    token::{token_pos::TokenPos, Token},
    token_collector::{
        number_collector::NumberCollector, operator_collector::OperatorCollector,
        special_collector::SpecialCollector, word_collector::WordCollector, TokenCollector,
    },
    unexpected_token::UnexpectedToken,
};

mod code_stream;
mod comments;
pub mod token;
mod token_collector;
pub mod unexpected_token;

pub struct Lexer {
    pub collectors: Vec<Box<dyn TokenCollector>>,
    pub code: CodeStream,
}

impl Lexer {
    pub fn new(code: String) -> Self {
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

    pub fn next_token(&mut self) -> Option<Result<Token, UnexpectedToken>> {
        Comments::skip(&mut self.code);

        let pos = self.code.get_pos();

        if self.code.is_eof() {
            return None;
        }

        for collector in self.collectors.iter_mut() {
            if let Some(token_value) = collector.try_next(&mut self.code) {
                return Some(Ok(Token::new(token_value, pos)));
            }
        }

        Some(Err(UnexpectedToken {
            value: self.code.accept().to_string(),
            pos,
        }))
    }
}

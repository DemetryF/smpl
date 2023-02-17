pub mod number_collector;
pub mod operator_collector;
pub mod special_collector;
pub mod word_collector;

use crate::{lexer::CodeStream, token::TokenValue};

pub use self::{
    number_collector::NumberCollector, operator_collector::OperatorCollector,
    special_collector::SpecialCollector, word_collector::WordCollector,
};

pub trait TokenCollector {
    fn try_next(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue>;
}

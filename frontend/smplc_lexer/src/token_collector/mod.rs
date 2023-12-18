pub mod number_collector;
pub mod special_collector;
pub mod word_collector;

use super::{CodeStream, TokenValue};

pub use self::{
    number_collector::NumberCollector, special_collector::SpecialCollector,
    word_collector::WordCollector,
};

pub trait TokenCollector {
    fn try_collect(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue>;
}

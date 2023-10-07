mod number_collector;
mod special_collector;
mod word_collector;

use crate::token::TokenValue;

use crate::code_stream::CodeStream;

pub use self::{
    number_collector::NumberCollector, special_collector::SpecialCollector,
    word_collector::WordCollector,
};

pub trait TokenCollector<'source> {
    fn try_collect(&self, code_stream: &mut CodeStream<'source>) -> Option<TokenValue<'source>>;
}

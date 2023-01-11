pub mod number_collector;
pub mod operator_collector;
pub mod special_collector;
pub mod word_collector;

use super::{code_stream::CodeStream, token::token_value::TokenValue};

pub trait TokenCollector {
    fn try_next<'code>(&mut self, code: &mut CodeStream<'code>) -> Option<TokenValue<'code>>;
}

mod number_collector;
mod special_collector;
mod word_collector;

use smplc_token::TokenValue;

use crate::code_stream::CodeStream;

pub trait TokenCollector {
    fn try_collect(&self, code_stream: &mut CodeStream) -> Option<TokenValue>;
}

pub mod number_collector;
pub mod special_collector;
pub mod word_collector;

use crate::{CodeStream, TokenValue};

pub use self::number_collector::NumberCollector;
pub use self::special_collector::SpecialCollector;
pub use self::word_collector::WordCollector;

pub trait TokenCollector {
    fn try_collect<'source>(
        &mut self,
        code_stream: &mut CodeStream<'source>,
    ) -> Option<TokenValue<'source>>;
}

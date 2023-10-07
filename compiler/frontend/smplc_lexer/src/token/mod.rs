pub use pos::Pos;
pub use token_value::{Literal, TokenValue};

mod pos;
mod token_value;

#[derive(Clone, Copy, Debug)]
pub struct Token<'source> {
    pub value: TokenValue<'source>,
    pub pos: Pos,
}

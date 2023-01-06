use derive_more::Constructor;

use self::{token_pos::TokenPos, token_value::TokenValue};

pub mod token_pos;
pub mod token_value;

#[derive(Debug, Constructor)]
pub struct Token {
    pub value: TokenValue,
    pub pos: TokenPos,
}

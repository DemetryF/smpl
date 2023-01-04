use self::{token_pos::TokenPos, token_value::TokenValue};

pub mod token_pos;
pub mod token_value;

#[derive(Debug)]
pub struct Token {
    pub value: TokenValue,
    pub pos: TokenPos,
}

impl Token {
    pub fn new(value: TokenValue, pos: TokenPos) -> Self {
        Self { value, pos }
    }
}

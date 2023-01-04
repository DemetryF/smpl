use self::{pos::Pos, token_value::TokenValue};

pub mod keyword;
pub mod operator;
pub mod pos;
pub mod special;
pub mod token_value;

#[derive(Debug)]
pub struct Token {
    pub value: TokenValue,
    pub pos: Pos,
}

impl Token {
    pub fn new(value: TokenValue, pos: Pos) -> Self {
        Self { value, pos }
    }
}

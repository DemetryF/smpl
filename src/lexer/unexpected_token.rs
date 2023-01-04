use super::token::token_pos::TokenPos;

#[derive(Debug)]
pub struct UnexpectedToken {
    pub value: String,
    pub pos: TokenPos,
}

use derive_more::Constructor;

use self::token_value::TokenValue;
use super::pos::Pos;

pub mod literal;
pub mod operator;
pub mod token_value;

#[derive(Debug, Constructor)]
pub struct Token<'code> {
    pub value: TokenValue<'code>,
    pub pos: Pos,
}

use derive_more::Constructor;

use self::token_value::TokenValue;
use super::pos::Pos;

pub mod operator;
pub mod token_value;

#[derive(Copy, Clone, Debug, Constructor)]
pub struct Token<'code> {
    pub value: TokenValue<'code>,
    pub pos: Pos,
}

use super::pos::Pos;
use derive_more::Constructor;

pub use self::operator::Operator;
pub use self::token_value::Literal;
pub use self::token_value::TokenValue;

mod fmt;
pub mod operator;
pub mod token_value;

#[derive(Clone, Debug, Constructor)]
pub struct Token {
    pub value: TokenValue,
    pub pos: Pos,
}

pub use pos::{Pos, Posed};
pub use token_value::{Literal, TokenValue};

mod pos;
mod token_value;

pub type Token<'source> = Posed<TokenValue<'source>>;

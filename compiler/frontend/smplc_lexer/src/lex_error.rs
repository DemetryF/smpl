use std::fmt;

use smplc_token::Pos;

pub struct LexError {
    pub pos: Pos,
    pub char: char,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected char '{}'", self.char)
    }
}

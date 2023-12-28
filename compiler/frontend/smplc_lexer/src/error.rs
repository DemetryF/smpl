use std::fmt;

use smplc_ast::Pos;

#[derive(Debug)]
pub struct LexError {
    pub char: char,
    pub pos: Pos,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected char '{}'", self.char)
    }
}

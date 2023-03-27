use derive_more::Constructor;

use crate::lexer::pos::Pos;

#[derive(Debug, Constructor)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedChar(char),
}

use crate::lexer::pos::Pos;

#[derive(Debug)]
pub enum Error {
    UnexpectedToken { value: String, pos: Pos },
}

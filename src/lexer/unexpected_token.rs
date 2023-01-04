use super::token::pos::Pos;

#[derive(Debug)]
pub struct UnexpectedToken {
    pub value: String,
    pub pos: Pos,
}

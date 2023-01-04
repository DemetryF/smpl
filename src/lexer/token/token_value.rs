use super::keyword::Keyword;
use super::operator::Operator;
use super::special::Special;

#[derive(Clone, Debug)]
pub enum TokenValue {
    Keyword(Keyword),
    Special(Special),
    Operator(Operator),

    Id(String),
    Bool(bool),
    Number(String),
}

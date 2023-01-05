use self::keyword::Keyword;
use self::operator::Operator;
use self::special::Special;

pub mod keyword;
pub mod operator;
pub mod special;

#[derive(Clone, Debug)]
pub enum TokenValue {
    Keyword(Keyword),
    Special(Special),
    Operator(Operator),

    Id(String),
    Bool(bool),
    Number(String),

    EOF,
}

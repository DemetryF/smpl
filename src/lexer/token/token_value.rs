use super::{literal::Literal, operator::Operator};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    Operator(Operator),
    Literal(Literal),
    Id(String),

    Eof,

    // Keywords
    Define,
    Else,
    Function,
    If,
    Return,
    While,

    // special chars
    Comma,
    Semicolon,
    OpeningParen,
    ClosingParen,
    OpeningBrace,
    ClosingBrace,
}

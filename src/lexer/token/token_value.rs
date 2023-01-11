use super::{literal::Literal, operator::Operator};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue<'code> {
    Operator(Operator),
    Literal(Literal<'code>),
    Id(&'code str),

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

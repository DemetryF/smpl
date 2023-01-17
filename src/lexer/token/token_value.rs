use super::operator::Operator;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenValue<'code> {
    Operator(Operator),
    Literal(Literal),
    Id(Id<'code>),

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    Bool(bool),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Id<'code>(pub &'code str);

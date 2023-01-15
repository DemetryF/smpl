use super::operator::Operator;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenValue<'code> {
    Operator(Operator),
    Literal(Literal),
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    Bool(bool),
}

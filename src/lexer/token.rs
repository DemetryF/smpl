use derive_more::Constructor;

use super::Pos;

#[derive(Debug, Constructor)]
pub struct Token {
    pub value: TokenValue,
    pub pos: Pos,
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    // keywords
    Return,
    While,
    Else,
    Let,
    Fn,
    If,

    // special chars
    Semicolon,
    Comma,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Assignment,
    Or,
    And,
    NotEqual,
    Equal,
    GreaterOrEqual,
    Greater,
    LessOrEqual,
    Less,
    Plus,
    Minus,
    Star,
    Slash,
    Not,

    // other
    Literal(Literal),
    Id(String),

    #[warn(clippy::upper_case_acronyms)]
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f32),
    Bool(bool),
}

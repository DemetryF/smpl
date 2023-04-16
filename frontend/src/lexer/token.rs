use derive_more::Constructor;

use super::Pos;

#[derive(Clone, Debug, Constructor)]
pub struct Token {
    pub value: TokenValue,
    pub pos: Pos,
}

#[derive(Clone, Debug, PartialEq)]
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

    EOF,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    Number(f32),
    Bool(bool),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(num) => write!(f, "{num}"),
            Literal::Bool(bool) => write!(f, "{bool}"),
        }
    }
}

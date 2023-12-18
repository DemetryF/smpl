use std::fmt::Display;

use super::Pos;

#[derive(Clone, Debug)]
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

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            TokenValue::Return => "return",
            TokenValue::While => "while",
            TokenValue::Else => "else",
            TokenValue::Let => "let",
            TokenValue::Fn => "fn",
            TokenValue::If => "if",
            TokenValue::Semicolon => ";",
            TokenValue::Comma => ",",
            TokenValue::LBrace => "{",
            TokenValue::RBrace => "}",
            TokenValue::LParen => "(",
            TokenValue::RParen => ")",
            TokenValue::Assignment => "=",
            TokenValue::Or => "|",
            TokenValue::And => "&",
            TokenValue::NotEqual => "!=",
            TokenValue::Equal => "==",
            TokenValue::GreaterOrEqual => ">=",
            TokenValue::Greater => ">",
            TokenValue::LessOrEqual => "<=",
            TokenValue::Less => "<",
            TokenValue::Plus => "+",
            TokenValue::Minus => "-",
            TokenValue::Star => "*",
            TokenValue::Slash => "/",
            TokenValue::Not => "!",
            TokenValue::Literal(literal) => {
                return write!(f, "{literal}");
            }
            TokenValue::Id(id) => &id,
            TokenValue::EOF => "\\0",
        };

        write!(f, "{value}")
    }
}

use std::fmt::Display;

use smplc_ast::{Literal, Pos};

#[derive(Clone, Copy, Debug)]
pub struct Token<'source> {
    pub value: TokenValue<'source>,
    pub pos: Pos,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TokenValue<'source> {
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
    Assign,
    Or,
    And,
    Ne,
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Plus,
    Minus,
    Star,
    Slash,
    Not,

    // other
    Literal(Literal),
    Id(&'source str),

    EOF,
}

impl<'source> Display for TokenValue<'source> {
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
            TokenValue::Assign => "=",
            TokenValue::Or => "|",
            TokenValue::And => "&",
            TokenValue::Ne => "!=",
            TokenValue::Eq => "==",
            TokenValue::Ge => ">=",
            TokenValue::Gt => ">",
            TokenValue::Le => "<=",
            TokenValue::Lt => "<",
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

use std::fmt::Display;

use smplc_ast::{Literal, Span, Type};

#[derive(Clone, Copy, Debug)]
pub struct Token<'source> {
    pub value: TokenValue<'source>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TokenValue<'source> {
    // keywords
    Break,
    Continue,
    Const,
    Else,
    Fn,
    If,
    Let,
    Return,
    While,

    // special chars
    Semicolon,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Assign,
    Arrow,
    Colon,
    Comma,
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
    Type(Type),
    Id(&'source str),

    EOF,
}

impl<'source> Display for TokenValue<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            TokenValue::Break => "break",
            TokenValue::Continue => "continue",
            TokenValue::Const => "const",
            TokenValue::Else => "else",
            TokenValue::Fn => "fn",
            TokenValue::If => "if",
            TokenValue::Let => "let",
            TokenValue::Return => "return",
            TokenValue::While => "while",

            TokenValue::Arrow => "->",
            TokenValue::Semicolon => ";",
            TokenValue::Colon => ":",
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
            TokenValue::Id(id) => id,
            TokenValue::EOF => "\\0",

            TokenValue::Literal(literal) => {
                return write!(f, "{literal}");
            }

            TokenValue::Type(ty) => {
                return write!(f, "{ty}");
            }
        };

        write!(f, "{value}")
    }
}

use std::fmt::Display;

use smplc_ast::{LiteralType, Span};

#[derive(Clone, Copy, Debug)]
pub struct Token<'source> {
    pub tag: TokenTag,
    pub span: Span,
    pub value: &'source str,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TokenTag {
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
    Literal(LiteralType),
    Id,

    EOF,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

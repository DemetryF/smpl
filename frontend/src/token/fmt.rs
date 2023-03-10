use super::{
    Literal,
    TokenValue::{self, *},
};
use std::fmt::Display;

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator(op) => write!(f, "{op}"),
            Literal(literal) => write!(f, "{literal}"),
            Id(id) => write!(f, "{id}"),

            Eof => write!(f, "end of input"),

            Define => write!(f, "define"),
            Else => write!(f, "else"),
            Function => write!(f, "function"),
            If => write!(f, "if"),
            Return => write!(f, "return"),
            While => write!(f, "while"),

            Comma => write!(f, ","),
            Semicolon => write!(f, ";"),
            OpeningParen => write!(f, "("),
            ClosingParen => write!(f, ")"),
            OpeningBrace => write!(f, "{{"),
            ClosingBrace => write!(f, "}}"),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::Bool(bool) => write!(f, "{bool}"),
        }
    }
}

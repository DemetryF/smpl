use std::fmt;

use smplc_ast::Pos;
use smplc_lexer::{Token, TokenValue};

pub type ParseResult<'source, T> = Result<T, ParseError<'source>>;

#[derive(Debug)]
pub struct ParseError<'source> {
    pub kind: ParseErrorKind<'source>,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum ParseErrorKind<'source> {
    UnexpectedToken(TokenValue<'source>),
    ReturnOutsideFunction,
    FunctionInBlock,
    BreakOutsideLoop,
    ContinueOutsideLoop,
}

impl<'source> ParseError<'source> {
    pub fn unexpected_token(Token { value, pos }: Token<'source>) -> Self {
        let kind = ParseErrorKind::UnexpectedToken(value);

        Self { kind, pos }
    }

    pub fn return_outside_function(pos: Pos) -> Self {
        let kind = ParseErrorKind::ReturnOutsideFunction;

        Self { kind, pos }
    }

    pub fn function_in_block(pos: Pos) -> Self {
        let kind = ParseErrorKind::FunctionInBlock;

        Self { kind, pos }
    }
}

impl fmt::Display for ParseErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken(token) => {
                write!(f, "unexpected token \"{token}\"")
            }

            Self::ReturnOutsideFunction => {
                write!(f, "using return outside the function")
            }

            Self::FunctionInBlock => {
                write!(f, "functions are not allowed in blocks")
            }

            Self::BreakOutsideLoop => {
                write!(f, "using break outside loop")
            }

            Self::ContinueOutsideLoop => {
                write!(f, "using continue outside loop")
            }
        }
    }
}

use smplc_token::{Pos, Token, TokenValue};

pub type ParseResult<T> = Result<T, ParseError>;

pub struct ParseError {
    pub kind: ParseErrorKind,
    pub pos: Pos,
}

pub enum ParseErrorKind {
    UnexpectedToken(TokenValue),
}

impl ParseError {
    pub fn unexpected_token(token: Token) -> Self {
        let kind = ParseErrorKind::UnexpectedToken(token.value);
        let pos = token.pos;

        Self { kind, pos }
    }
}

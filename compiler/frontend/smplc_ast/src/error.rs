use smplc_token::{Pos, Token, TokenValue};

pub type ParseResult<'source, T> = Result<T, ParseError<'source>>;

pub struct ParseError<'source> {
    pub kind: ParseErrorKind<'source>,
    pub pos: Pos,
}

pub enum ParseErrorKind<'source> {
    UnexpectedToken(TokenValue<'source>),
}

impl<'source> ParseError<'source> {
    pub fn unexpected_token(token: Token<'source>) -> Self {
        let kind = ParseErrorKind::UnexpectedToken(token.value);
        let pos = token.pos;

        Self { kind, pos }
    }
}

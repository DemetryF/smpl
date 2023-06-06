use smplc_token::{Pos, Token, TokenValue};

pub type ParseResult<'source, T> = Result<T, ParseError<'source>>;

pub struct ParseError<'source> {
    pub kind: ParseErrorKind<'source>,
    pub pos: Pos,
}

pub enum ParseErrorKind<'source> {
    UnexpectedToken(TokenValue<'source>),

    BreakOutsideCycle,
    ContinueOutsideCycle,
    ReturnOutsideCycle,
}

impl<'source> ParseError<'source> {
    pub fn unexpected_token(token: Token<'source>) -> Self {
        let kind = ParseErrorKind::UnexpectedToken(token.value);
        let pos = token.pos;

        Self { kind, pos }
    }

    pub fn break_outside_cycle(pos: Pos) -> Self {
        let kind = ParseErrorKind::BreakOutsideCycle;

        Self { kind, pos }
    }

    pub fn continue_outside_cycle(pos: Pos) -> Self {
        let kind = ParseErrorKind::ContinueOutsideCycle;

        Self { kind, pos }
    }

    pub fn return_outside_function(pos: Pos) -> Self {
        let kind = ParseErrorKind::ReturnOutsideCycle;

        Self { kind, pos }
    }
}

use crate::{
    error::*,
    lexer::Lexer,
    token::{Token, TokenValue},
};

pub struct TokenStream<'code> {
    pub lexer: Lexer<'code>,
    pub in_function: bool,
    pub errors: Vec<Error>,

    current: Token,
    following: Option<Token>,
}

impl<'code> TokenStream<'code> {
    pub fn new(code: &'code str) -> Result<Self> {
        let mut lexer = Lexer::new(code);

        Ok(Self {
            current: lexer.next_token()?,
            errors: Vec::new(),
            following: None,
            in_function: false,
            lexer,
        })
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn following(&mut self) -> Result<&Token> {
        if self.following.is_some() {
            return Ok(self.following.as_ref().unwrap());
        }

        let next_token = self.lexer.next_token()?;
        self.following = Some(next_token);
        self.following()
    }

    pub fn skip(&mut self) -> Result<Token> {
        let token = self.current.clone();

        match self.following.take() {
            Some(token) => self.current = token,
            None => self.current = self.lexer.next_token()?,
        }

        Ok(token)
    }

    pub fn check(&self, value: &TokenValue) -> bool {
        &self.current().value == value
    }

    pub fn accept(&mut self, value: &TokenValue) -> Result<()> {
        if self.check(value) {
            self.skip()?;
            return Ok(());
        }

        let current = self.current.clone();

        Err(Error::new(
            ErrorKind::UnexpectedToken(current.value),
            current.pos,
        ))
    }

    pub fn is_end(&self) -> bool {
        self.check(&TokenValue::Eof)
    }

    pub fn skip_if(&mut self, value: &TokenValue) -> Result<Option<Token>> {
        if self.check(value) {
            return Ok(Some(self.skip()?));
        }

        Ok(None)
    }
}

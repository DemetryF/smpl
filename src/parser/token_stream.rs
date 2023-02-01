use crate::{
    error::*,
    lexer::{Lexer, Token, TokenValue},
};

pub struct TokenStream<'code> {
    pub lexer: Lexer<'code>,

    current: Token,
    following: Option<Token>,

    pub in_function: bool,

    pub errors: Vec<Error>,
}

impl<'code> TokenStream<'code> {
    pub fn new(code: &'code str) -> Self {
        let mut lexer = Lexer::new(code);
        let mut errors: Vec<Error> = Vec::new();

        let current = loop {
            match lexer.next_token() {
                Ok(token) => break token,
                Err(error) => errors.push(error),
            }
        };

        Self {
            lexer,
            errors,
            current,
            following: None,
            in_function: false,
        }
    }

    fn get_next_from_lexer(&mut self) -> Token {
        match self.lexer.next_token() {
            Ok(token) => token,
            Err(error) => {
                self.errors.push(error);
                self.get_next_from_lexer()
            }
        }
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn following(&mut self) -> &Token {
        if self.following.is_some() {
            return self.following.as_ref().unwrap();
        }

        let next_token = self.get_next_from_lexer();
        self.following = Some(next_token);
        self.following()
    }

    pub fn skip(&mut self) -> Token {
        let token = self.current.clone();

        match self.following.take() {
            Some(token) => self.current = token,
            None => self.current = self.get_next_from_lexer(),
        }

        token
    }

    pub fn check(&self, value: &TokenValue) -> bool {
        &self.current().value == value
    }

    pub fn accept(&mut self, value: &TokenValue) {
        if self.check(value) {
            self.skip();
            return;
        }

        panic!("{}", "sperma")
    }

    pub fn is_end(&self) -> bool {
        self.check(&TokenValue::Eof)
    }

    pub fn skip_if(&mut self, value: &TokenValue) -> Option<Token> {
        if self.check(value) {
            return Some(self.skip());
        }

        None
    }
}

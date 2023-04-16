use crate::{
    error::Error,
    lexer::{Lexer, Pos, Token, TokenValue},
};

pub struct TokenStream {
    tokens: Vec<Token>,
    index: usize,

    pub in_function: bool,
}

impl TokenStream {
    pub fn new(code: &str) -> Result<Self, Vec<Error>> {
        let mut lexer = Lexer::new(code);

        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        loop {
            let token = lexer.next_token();

            match token {
                Ok(
                    token @ Token {
                        value: TokenValue::EOF,
                        ..
                    },
                ) => {
                    tokens.push(token);
                    break;
                }

                Ok(_) if !errors.is_empty() => {}

                Ok(token) => {
                    tokens.push(token);
                }
                Err(error) => errors.push(error),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            tokens,
            index: 0,
            in_function: false,
        })
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn check(&self, value: TokenValue) -> bool {
        !self.is_end() && self.current().value == value
    }

    pub fn consume(&mut self, value: TokenValue) -> Result<(), Error> {
        if self.check(value) {
            self.next();

            return Ok(());
        }

        Err(self.unexpected_token())
    }

    pub fn try_consume(&mut self, value: TokenValue) -> bool {
        if self.check(value) {
            self.next();

            return true;
        }

        false
    }

    pub fn next(&mut self) -> Token {
        let token = self.current().clone();

        self.index += 1;

        token
    }

    pub fn unexpected_token(&self) -> Error {
        Error::unexpected_token(self.current().clone())
    }

    pub fn get_pos(&self) -> Pos {
        self.current().pos
    }

    pub fn is_end(&self) -> bool {
        self.current().value == TokenValue::EOF
    }
}

use crate::lexer::{
    token::{token_value::TokenValue, Token},
    unexpected_token::UnexpectedToken,
    Lexer,
};

pub struct TokenStream {
    lexer: Lexer,

    current: Token,
    following: Option<Token>,

    pub errors: Vec<UnexpectedToken>,
}

impl TokenStream {
    pub fn new(code: String) -> Self {
        let mut lexer = Lexer::new(code);
        let mut errors: Vec<UnexpectedToken> = Vec::new();

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

        println!("{:?}", self.current().value);
        println!("{:?}", value);

        panic!("pizdaus")
    }
}

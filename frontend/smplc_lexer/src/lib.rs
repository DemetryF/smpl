mod code_stream;
mod comment_handler;
mod pos;
mod token;
mod token_collector;

mod error;
mod lexer;
#[cfg(test)]
mod tests;

use lexer::Lexer;

pub use self::{
    error::LexError,
    pos::Pos,
    token::{Literal, Token, TokenValue},
};

use self::code_stream::CodeStream;

pub fn lex(code: &str) -> Result<Vec<Token>, LexError> {
    let lexer = Lexer::new(code);
    let tokens = lexer.collect::<Result<Vec<_>, _>>()?;

    Ok(tokens)
}

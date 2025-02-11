mod comment_handler;
mod cursor;
mod error;
mod lexer;
mod token;
mod token_collector;

#[cfg(test)]
mod tests;

pub use self::error::LexError;
pub use self::token::{Token, TokenValue};

use cursor::Cursor;
use lexer::Lexer;

pub fn lex(code: &str) -> Result<Vec<Token>, LexError> {
    let lexer = Lexer::new(code);
    let tokens = lexer.collect::<Result<Vec<_>, _>>()?;

    Ok(tokens)
}

mod cursor;
mod error;
mod lexer;
mod number;
mod skip;
mod token;
mod word;

mod specials;
#[cfg(test)]
mod tests;

pub use self::error::LexError;
pub use self::token::{Token, TokenValue};

use lexer::Lexer;

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let lexer = Lexer::new(source);
    let tokens = lexer.collect::<Result<Vec<_>, _>>()?;

    Ok(tokens)
}

mod cursor;
mod error;
mod lexer;
mod number;
mod skip;
mod specials;
mod token;
mod word;

#[cfg(test)]
mod tests;

pub use error::LexError;
pub use lexer::Lexer;
pub use token::{Token, TokenValue};

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let lexer = Lexer::new(source);
    let tokens = lexer.collect::<Result<Vec<_>, _>>()?;

    Ok(tokens)
}

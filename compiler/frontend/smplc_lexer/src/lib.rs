mod code_stream;
mod comment_handler;
mod error;
mod lexer;
mod token;
mod token_collector;

#[cfg(test)]
mod tests;

pub use self::error::LexError;
pub use self::token::{Token, TokenValue};

use code_stream::CodeStream;
use lexer::Lexer;

pub fn lex(code: &str) -> Result<Vec<Token>, LexError> {
    let lexer = Lexer::new(code);
    let tokens = lexer.collect::<Result<Vec<_>, _>>()?;

    Ok(tokens)
}

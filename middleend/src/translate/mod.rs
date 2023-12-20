mod block;
mod expr;
mod statement;

use crate::{Error, Translator};

pub trait Translate<'source, Return = ()> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<Return, Error<'source>>;
}

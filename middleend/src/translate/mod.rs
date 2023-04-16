mod block;
mod expr;
mod statement;

use crate::{Error, Translator};

pub trait Translate<Return = ()> {
    fn translate(self, translator: &mut Translator) -> Result<Return, Error>;
}

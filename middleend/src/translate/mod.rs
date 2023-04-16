mod block;
mod expr;
mod statement;

use crate::Translator;

pub trait Translate<Return = ()> {
    fn translate(self, translator: &mut Translator) -> Return;
}

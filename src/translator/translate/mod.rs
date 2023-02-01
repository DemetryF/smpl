use super::Translator;
use crate::ast::Atom;

pub mod block;
pub mod expr;
pub mod statement;

pub trait Translate {
    fn translate(self, translator: &mut Translator) -> Option<Atom>;
}

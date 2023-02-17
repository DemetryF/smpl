use frontend::ast::Atom;

use super::Translator;

pub mod block;
pub mod expr;
pub mod statement;

pub trait Translate {
    fn translate(self, translator: &mut Translator) -> Option<Atom>;
}

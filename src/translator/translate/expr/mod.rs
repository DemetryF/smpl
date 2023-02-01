use super::Translate;
use crate::{
    ast::{Atom, Expr},
    translator::Translator,
};

pub mod binary;
pub mod call;
pub mod unary;

impl Translate for Expr {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        match self {
            Self::Binary(binary) => binary.translate(translator),
            Self::Unary(unary) => unary.translate(translator),
            Self::Call(call) => call.translate(translator),
            Self::Atom(atom) => Some(atom),
        }
    }
}

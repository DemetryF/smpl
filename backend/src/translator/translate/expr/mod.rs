use frontend::ast::{Atom, Expr};

use super::{Translate, Translator};

pub mod call;
pub mod infix;
pub mod prefix;

impl Translate for Expr {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        match self {
            Self::Infix(infix) => infix.translate(translator),
            Self::Prefix(prefix) => prefix.translate(translator),
            Self::Call(call) => call.translate(translator),
            Self::Atom(atom) => Some(atom),
        }
    }
}

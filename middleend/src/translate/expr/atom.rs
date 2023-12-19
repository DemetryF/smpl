use crate::{instruction::Atom, translate::Translate, Error, Translator};

impl Translate<Atom> for smplc_ast::Atom {
    fn translate(self, translator: &mut Translator) -> Result<Atom, Error> {
        match self {
            smplc_ast::Atom::Id(id) => translator
                .scopes
                .get_variable(id)
                .map(|variable| Atom::from(variable.id)),

            smplc_ast::Atom::Literal(literal) => Ok(Atom::from(literal)),
        }
    }
}

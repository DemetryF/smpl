use crate::{instruction::Atom, translate::Translate, Error, Translator};

impl<'source> Translate<'source, Atom> for smplc_ast::Atom<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<Atom, Error<'source>> {
        match self {
            smplc_ast::Atom::Id(id) => translator
                .scopes
                .get_variable(id)
                .map(|variable| Atom::from(variable.id)),

            smplc_ast::Atom::Literal(literal) => Ok(Atom::from(literal)),
        }
    }
}

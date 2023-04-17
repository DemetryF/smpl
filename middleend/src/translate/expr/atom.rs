use crate::{instruction::Atom, translate::Translate, Error, Translator};

impl Translate<Atom> for frontend::ast::Atom {
    fn translate(self, translator: &mut Translator) -> Result<Atom, Error> {
        match self {
            frontend::ast::Atom::Id(id) => translator
                .scopes
                .get_variable(id)
                .map(|variable| Atom::from(variable.id)),

            frontend::ast::Atom::Literal(literal) => Ok(Atom::from(literal)),
        }
    }
}

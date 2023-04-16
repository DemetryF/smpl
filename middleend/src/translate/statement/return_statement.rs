use frontend::ast::ReturnStatement;

use crate::{
    instruction::{Atom, Return},
    translate::Translate,
    Error, Translator,
};

impl Translate for ReturnStatement {
    fn translate(self, translator: &mut Translator) -> Result<(), Error> {
        let value = self
            .0
            .map(|expr| expr.translate(translator))
            .unwrap_or(Ok(Atom::Number(0.0)))?;

        translator.code.push(Return { value });

        Ok(())
    }
}

use frontend::ast::ReturnStatement;

use crate::{
    instruction::{Atom, Return},
    translate::Translate,
    Translator,
};

impl Translate for ReturnStatement {
    fn translate(self, translator: &mut Translator) {
        let value = self
            .0
            .map(|expr| expr.translate(translator))
            .unwrap_or(Atom::Number(0.0));

        translator.code.push(Return { value });
    }
}

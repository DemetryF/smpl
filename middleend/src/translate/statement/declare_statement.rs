use frontend::ast::DeclareStatement;

use crate::instruction::*;
use crate::{translate::Translate, Translator};

impl Translate for DeclareStatement {
    fn translate(self, translator: &mut Translator) {
        let value = if let Some(expr) = self.init_expr {
            expr.translate(translator)
        } else {
            Atom::Number(0.0)
        };

        let result = match translator.scopes.add_variable(self.id) {
            Ok(id) => id,
            Err(error) => return translator.errors.push(error),
        };

        translator.code.push(Copy { result, value })
    }
}

use frontend::ast::DeclareStatement;

use crate::{instruction::*, Error};
use crate::{translate::Translate, Translator};

impl Translate for DeclareStatement {
    fn translate(self, translator: &mut Translator) -> Result<(), Error> {
        let value = if let Some(expr) = self.init_expr {
            expr.translate(translator)?
        } else {
            Atom::Number(0.0)
        };

        let result = translator.scopes.add_variable(self.id)?;

        translator.code.push(Copy { result, value });

        Ok(())
    }
}

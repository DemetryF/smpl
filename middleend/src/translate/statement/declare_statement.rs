use smplc_ast::DeclareStatement;
use smplc_ir::{Atom, Copy};

use crate::translate::Translate;
use crate::{Error, Translator};

impl<'source> Translate<'source> for DeclareStatement<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<(), Error<'source>> {
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

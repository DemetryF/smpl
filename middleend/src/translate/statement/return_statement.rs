use smplc_ast::ReturnStatement;

use crate::{instruction::Return, translate::Translate, Error, Translator};

impl<'source> Translate<'source> for ReturnStatement<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<(), Error<'source>> {
        let value = if let Some(expr) = self.0 {
            Some(expr.translate(translator)?)
        } else {
            None
        };

        translator.code.push(Return { value });

        Ok(())
    }
}

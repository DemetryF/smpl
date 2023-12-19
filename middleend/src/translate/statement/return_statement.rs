use smplc_ast::ReturnStatement;

use crate::{instruction::Return, translate::Translate, Error, Translator};

impl Translate for ReturnStatement {
    fn translate(self, translator: &mut Translator) -> Result<(), Error> {
        let value = if let Some(expr) = self.0 {
            Some(expr.translate(translator)?)
        } else {
            None
        };

        translator.code.push(Return { value });

        Ok(())
    }
}

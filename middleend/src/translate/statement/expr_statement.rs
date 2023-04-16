use frontend::ast::ExprStatement;

use crate::{translate::Translate, Error, Translator};

impl Translate for ExprStatement {
    fn translate(self, translator: &mut Translator) -> Result<(), Error> {
        self.0.translate(translator)?;

        Ok(())
    }
}

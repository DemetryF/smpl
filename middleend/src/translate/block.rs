use smplc_ast::Block;

use crate::{Error, Translator};

use super::Translate;

impl<'source> Translate<'source> for Block<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<(), Error<'source>> {
        translator.scopes.fork();

        for stmt in self.stmts {
            stmt.translate(translator)?;
        }

        translator.scopes.exit();

        Ok(())
    }
}

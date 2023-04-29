use frontend::ast::Block;

use crate::{Error, Translator};

use super::Translate;

impl Translate for Block {
    fn translate(self, translator: &mut Translator) -> Result<(), Error> {
        translator.scopes.fork();

        for stmt in self.stmts {
            stmt.translate(translator)?;
        }

        translator.scopes.exit();

        Ok(())
    }
}

use frontend::ast::Block;

use crate::Translator;

use super::Translate;

impl Translate for Block {
    fn translate(self, translator: &mut Translator) {
        translator.scopes.fork();

        for stmt in self.stmts {
            stmt.translate(translator);
        }

        translator.scopes.exit();
    }
}

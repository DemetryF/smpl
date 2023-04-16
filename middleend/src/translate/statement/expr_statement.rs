use frontend::ast::ExprStatement;

use crate::{translate::Translate, Translator};

impl Translate for ExprStatement {
    fn translate(self, translator: &mut Translator) {
        self.0.translate(translator);
    }
}

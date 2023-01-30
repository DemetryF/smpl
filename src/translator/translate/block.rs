use super::Translate;
use crate::{
    parser::ast::{block::Block, expr::Atom},
    translator::Translator,
};

impl Translate for Block {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        for stmt in self.0 {
            stmt.translate(translator);
        }

        None
    }
}

use frontend::ast::{Atom, Block};

use super::{Translate, Translator};

impl Translate for Block {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        for stmt in self.0 {
            stmt.translate(translator);
        }

        None
    }
}

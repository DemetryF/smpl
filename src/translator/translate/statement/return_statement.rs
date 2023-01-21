use crate::{
    parser::ast::{expr::Atom, statement::return_statement::ReturnStatement},
    translator::{instruction::Instruction, translate::Translate, Translator},
};

impl Translate for ReturnStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        if let Some(expr) = self.0 {
            let value = expr.translate(translator).expect("");
            translator.push(Instruction::Return(Some(value)));
        } else {
            translator.push(Instruction::Return(None));
        }

        None
    }
}

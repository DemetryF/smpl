use crate::{
    parser::ast::{expr::Atom, statement::return_statement::ReturnStatement},
    translator::{instruction::Instruction, translate::Translate, Translator},
};

impl Translate for ReturnStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let value = self.0.map(|x| x.translate(translator));
        translator.push(Instruction::Return(value.unwrap()));

        None
    }
}

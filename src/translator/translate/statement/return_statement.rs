use crate::{
    parser::ast::{Atom, ReturnStatement},
    translator::{
        instruction::{Instruction, Return},
        translate::Translate,
        Translator,
    },
};

impl Translate for ReturnStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let value = self.0.map(|x| x.translate(translator));
        translator.push(Instruction::Return(Return(value.unwrap())));

        None
    }
}

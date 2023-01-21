use crate::{
    parser::ast::{expr::Atom, statement::function_statement::FunctionStatement},
    translator::{
        instruction::{Instruction, Label},
        translate::Translate,
        Translator,
    },
};

impl Translate for FunctionStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        translator.push(Instruction::Label(Label(self.id)));

        for arg in self.args {
            translator.push(Instruction::Pop(arg))
        }

        self.body.translate(translator);

        None
    }
}

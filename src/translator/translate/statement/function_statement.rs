use crate::{
    parser::ast::{expr::Atom, statement::function_statement::FunctionStatement},
    translator::{
        instruction::{Instruction, Label},
        translate::Translate,
        Translator,
    },
};

impl Translate for FunctionStatement {
    fn translate(mut self, translator: &mut Translator) -> Option<Atom> {
        translator.push(Instruction::Label(Label(self.id)));

        self.args.reverse();

        for arg in self.args {
            translator.push(Instruction::Pop(arg))
        }

        self.body.translate(translator);

        None
    }
}

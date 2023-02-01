use crate::{
    ast::{Atom, FunctionStatement},
    translator::{
        instruction::{Instruction, Label, Pop, Return},
        translate::Translate,
        Translator,
    },
};

impl Translate for FunctionStatement {
    fn translate(mut self, translator: &mut Translator) -> Option<Atom> {
        translator.push(Instruction::Label(Label(self.id.value)));

        self.args.reverse();

        for arg in self.args {
            translator.push(Instruction::Pop(Pop(arg.value)))
        }

        self.body.translate(translator);

        if !self.has_return {
            translator.push(Instruction::Return(Return(None)))
        }

        None
    }
}

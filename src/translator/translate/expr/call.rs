use crate::{
    parser::ast::{Atom, Call as PCall, Expr},
    translator::{
        instruction::{Call, Instruction, Push},
        translate::Translate,
        Translator,
    },
};

impl Translate for PCall {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let args_count = self.args.len();
        let result = translator.get_temp_var();

        Self::translate_args(self.args, translator);

        translator.push(Instruction::Call(Call::new(
            result.clone(),
            self.id.value,
            args_count,
        )));

        Some(result)
    }
}

impl PCall {
    fn translate_args(args: Vec<Expr>, translator: &mut Translator) {
        for arg in args {
            let arg_result = arg.translate(translator).unwrap();
            translator.push(Instruction::Push(Push(arg_result)));
        }
    }
}

use crate::{
    parser::ast::expr::{call::Call, Atom, Expr},
    translator::{instruction::Instruction, translate::Translate, Translator},
};

impl Translate for Call {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let args_count = self.args.len();
        let result = translator.get_temp_var();

        Self::translate_args(self.args, translator);

        translator.push(Instruction::Call {
            result: result.clone(),
            name: self.id.value,
            args_count,
        });

        Some(Atom::Temp(result))
    }
}

impl Call {
    fn translate_args(args: Vec<Expr>, translator: &mut Translator) {
        for arg in args {
            let arg_result = arg.translate(translator).unwrap();
            translator.push(Instruction::Push(arg_result));
        }
    }
}

use crate::{
    parser::ast::expr::{call::Call, Atom},
    translator::{instruction::Instruction, translate::Translate, Translator},
};

impl Translate for Call {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let args_count = self.args.len();
        let result = translator.get_temp_var();

        for arg in self.args {
            let arg_result = arg.translate(translator).expect("");
            translator.push(Instruction::Push(arg_result));
        }

        translator.push(Instruction::Call {
            result: result.clone(),
            name: self.id,
            args_count,
        });

        Some(Atom::Id(result))
    }
}

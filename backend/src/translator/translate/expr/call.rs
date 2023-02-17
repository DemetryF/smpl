use frontend::ast::{Atom, Expr};

use crate::translator::{
    instruction::{Call, Instruction, Push},
    Translate, Translator,
};

impl Translate for frontend::ast::Call {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let args_count = self.args.len();
        let result = translator.get_temp_var();

        translate_args(self.args, translator);

        translator.push(Instruction::Call(Call::new(
            result.clone(),
            self.id.value,
            args_count,
        )));

        Some(result)
    }
}

fn translate_args(args: Vec<Expr>, translator: &mut Translator) {
    for arg in args {
        let arg_result = arg.translate(translator).unwrap();
        translator.push(Instruction::Push(Push(arg_result)));
    }
}

use crate::{
    parser::ast::expr::{unary::Unary as PUnary, Atom},
    translator::{
        instruction::{Instruction, Unary},
        translate::Translate,
        Translator,
    },
};

impl Translate for PUnary {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let result = translator.get_temp_var();

        let operand = self.rhs.translate(translator).unwrap();

        translator.push(Instruction::Unary(Unary::new(
            result.clone(),
            self.op,
            operand,
        )));

        Some(result)
    }
}

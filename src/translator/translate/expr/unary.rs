use crate::{
    parser::ast::expr::{unary::Unary, Atom},
    translator::{instruction::Instruction, translate::Translate, Translator},
};

impl Translate for Unary {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let result = translator.get_temp_var();

        let operand = self.rhs.translate(translator).unwrap();

        translator.push(Instruction::Unary {
            result: result.clone(),
            op: self.op,
            operand,
        });

        Some(Atom::Temp(result))
    }
}

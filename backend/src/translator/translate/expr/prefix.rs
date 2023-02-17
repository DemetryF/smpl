use frontend::ast::{Atom, Prefix};

use crate::translator::{
    instruction::{Instruction, Unary},
    Translate, Translator,
};

impl Translate for Prefix {
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

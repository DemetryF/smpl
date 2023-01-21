use crate::{
    parser::ast::expr::{Atom, Binary},
    translator::{instruction::Instruction, translate::Translate, Translator},
};

impl Translate for Binary {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let result = translator.get_temp_var();

        let left = self.lhs.translate(translator).expect("");
        let right = self.rhs.translate(translator).expect("");

        translator.push(Instruction::Binary {
            result: result.clone(),
            left,
            op: self.op,
            right,
        });

        Some(Atom::Id(result))
    }
}

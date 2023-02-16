use crate::{
    ast::{Atom, Expr, Infix},
    lexer::Operator,
    translator::{
        instruction::{Assign, Binary, Instruction},
        translate::Translate,
        Translator,
    },
};

impl Translate for Infix {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        match self.op {
            Operator::MultiplicationAssignment
            | Operator::SubtractionAssignment
            | Operator::AdditionAssignment
            | Operator::Assignment
            | Operator::DivisionAssignment => self.translate_assignment(translator),

            _ => self.translate_no_assignment(translator),
        }
    }
}

impl Infix {
    fn translate_assignment(self, translator: &mut Translator) -> Option<Atom> {
        let what = self.rhs.translate(translator).unwrap();

        let Expr::Atom(to) = self.lhs.as_ref() else {
            panic!();
        };
        let to = to.to_owned();

        translator.push(Instruction::Assign(Assign::new(what, self.op, to.clone())));

        Some(to)
    }

    fn translate_no_assignment(self, translator: &mut Translator) -> Option<Atom> {
        let result = translator.get_temp_var();

        let left = self.lhs.translate(translator).unwrap();
        let right = self.rhs.translate(translator).unwrap();

        translator.push(Instruction::Binary(Binary::new(
            result.clone(),
            left,
            self.op,
            right,
        )));

        Some(result)
    }
}

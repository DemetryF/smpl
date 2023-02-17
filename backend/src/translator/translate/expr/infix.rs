use frontend::{
    ast::{Atom, Expr, Infix},
    token::Operator,
};

use crate::translator::{
    instruction::{Assign, Binary, Instruction},
    Translate, Translator,
};

impl Translate for Infix {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        match self.op {
            Operator::MultiplicationAssignment
            | Operator::SubtractionAssignment
            | Operator::AdditionAssignment
            | Operator::Assignment
            | Operator::DivisionAssignment => translate_assignment(self, translator),

            _ => translate_no_assignment(self, translator),
        }
    }
}

fn translate_assignment(infix: Infix, translator: &mut Translator) -> Option<Atom> {
    let what = infix.rhs.translate(translator).unwrap();

    let Expr::Atom(to) = infix.lhs.as_ref() else {
            panic!();
        };
    let to = to.to_owned();

    translator.push(Instruction::Assign(Assign::new(what, infix.op, to.clone())));

    Some(to)
}

fn translate_no_assignment(infix: Infix, translator: &mut Translator) -> Option<Atom> {
    let result = translator.get_temp_var();

    let left = infix.lhs.translate(translator).unwrap();
    let right = infix.rhs.translate(translator).unwrap();

    translator.push(Instruction::Binary(Binary::new(
        result.clone(),
        left,
        infix.op,
        right,
    )));

    Some(result)
}

use crate::{
    lexer::token::operator::Operator,
    parser::ast::expr::{Atom, Binary, Expr},
    translator::{instruction::Instruction, translate::Translate, Translator},
};

impl Translate for Binary {
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

impl Binary {
    fn translate_assignment(self, translator: &mut Translator) -> Option<Atom> {
        let what = self.rhs.translate(translator).unwrap();

        let Expr::Atom(to) = self.lhs.as_ref() else {
            panic!();
        };
        let to = to.to_owned();

        translator.push(Instruction::Assign {
            what,
            op: self.op,
            to: to.clone(),
        });

        Some(to)
    }

    fn translate_no_assignment(self, translator: &mut Translator) -> Option<Atom> {
        let result = translator.get_temp_var();

        let left = self.lhs.translate(translator).unwrap();
        let right = self.rhs.translate(translator).unwrap();

        translator.push(Instruction::Binary {
            result: result.clone(),
            left,
            op: self.op,
            right,
        });

        Some(result)
    }
}

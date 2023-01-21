use crate::{
    lexer::token::operator::Operator,
    parser::ast::expr::{Atom, Binary, Expr},
    translator::{instruction::Instruction, translate::Translate, Translator},
};

impl Translate for Binary {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        match self.op {
            op @ (Operator::Assignment
            | Operator::AdditionAssignment
            | Operator::SubtractionAssignment
            | Operator::MultiplicationAssignment
            | Operator::DivisionAssignment) => {
                let what = self.rhs.translate(translator).expect("");

                let Expr::Atom(Atom::Id(to)) = self.lhs.as_ref() else {
                    panic!();
                };

                translator.push(Instruction::Assign {
                    what,
                    op,
                    to: to.clone(),
                });
                Some(Atom::Id(to.clone()))
            }

            _ => {
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
    }
}

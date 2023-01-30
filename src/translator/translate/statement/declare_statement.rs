use crate::{
    lexer::{Literal, Operator},
    parser::ast::{Atom, DeclareStatement},
    translator::{
        instruction::{Assign, Instruction},
        translate::Translate,
        Translator,
    },
};

impl Translate for DeclareStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let value = if let Some(expr) = self.expr {
            expr.translate(translator).unwrap()
        } else {
            Atom::Literal(Literal::Number(0.0))
        };

        translator.push(Instruction::Assign(Assign::new(
            value,
            Operator::Assignment,
            Atom::Id(self.id),
        )));

        None
    }
}

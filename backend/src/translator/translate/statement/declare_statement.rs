use frontend::{
    ast::{Atom, DeclareStatement},
    token::{Literal, Operator},
};

use crate::translator::{
    instruction::{Assign, Instruction},
    Translate, Translator,
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

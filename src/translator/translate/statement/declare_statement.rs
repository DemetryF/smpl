use crate::{
    lexer::token::{operator::Operator, token_value::Literal},
    parser::ast::{expr::Atom, statement::declare_statement::DeclareStatement},
    translator::{instruction::Instruction, translate::Translate, Translator},
};

impl Translate for DeclareStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let value = if let Some(expr) = self.expr {
            expr.translate(translator).expect("")
        } else {
            Atom::Literal(Literal::Number(0.0))
        };

        translator.push(Instruction::Assign {
            what: value,
            op: Operator::Assignment,
            to: self.id,
        });

        None
    }
}

use crate::{
    parser::ast::{expr::Atom, statement::Statement},
    translator::Translator,
};

use super::Translate;

pub mod declare_statement;
pub mod function_statement;
pub mod if_statement;
pub mod return_statement;
pub mod while_statement;

impl Translate for Statement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        match self {
            Self::Expr(expr) => expr.translate(translator),
            Self::Declare(declare) => declare.translate(translator),
            Self::Function(function) => function.translate(translator),
            Self::If(if_stmt) => if_stmt.translate(translator),
            Self::Return(return_stmt) => return_stmt.translate(translator),
            Self::While(while_stmt) => while_stmt.translate(translator),
        };

        None
    }
}

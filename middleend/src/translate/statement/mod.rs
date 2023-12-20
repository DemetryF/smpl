mod declare_statement;
mod expr_statement;
mod function_statement;
mod if_statement;
mod return_statement;
mod while_statement;

use smplc_ast::Statement;

use crate::{Error, Translator};

use super::Translate;

impl<'source> Translate<'source> for Statement<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<(), Error<'source>> {
        match self {
            Statement::Declare(declare_statement) => declare_statement.translate(translator),
            Statement::Function(function_statement) => function_statement.translate(translator),
            Statement::If(if_statement) => if_statement.translate(translator),
            Statement::While(while_statement) => while_statement.translate(translator),
            Statement::Expr(expr_statement) => expr_statement.translate(translator),
            Statement::Return(return_statement) => return_statement.translate(translator),
        }
    }
}

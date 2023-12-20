use smplc_ast::{BinOp, Expr, ExprStatement};

use crate::{instruction::Copy, translate::Translate, Error, Translator};

impl<'source> Translate<'source> for ExprStatement<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<(), Error<'source>> {
        match self.0 {
            smplc_ast::Expr::Infix {
                lhs,
                op: BinOp::Assign,
                rhs,
            } => {
                let Expr::Atom(smplc_ast::Atom::Id(id)) = lhs.as_ref() else {
                    return Err(Error::expected_lvalue());
                };

                let result = translator.scopes.get_variable(id.clone())?.id;
                let value = rhs.translate(translator)?;

                translator.code.push(Copy { result, value });
            }

            smplc_ast::Expr::Call(call) => call.translate(translator)?,

            _ => {}
        }

        Ok(())
    }
}

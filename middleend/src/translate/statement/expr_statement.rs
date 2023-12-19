use smplc_ast::{BinOp, Expr, ExprStatement};

use crate::{instruction::Copy, translate::Translate, Error, Translator};

impl Translate for ExprStatement {
    fn translate(self, translator: &mut Translator) -> Result<(), Error> {
        match self.0 {
            smplc_ast::Expr::Infix {
                lhs,
                op: BinOp::Assignment,
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

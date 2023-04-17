use frontend::ast::{BinOp, Expr, ExprStatement};

use crate::{
    instruction::{Copy, Id},
    translate::Translate,
    Error, Translator,
};

impl Translate for ExprStatement {
    fn translate(self, translator: &mut Translator) -> Result<(), Error> {
        match self.0 {
            frontend::ast::Expr::Infix {
                lhs,
                op: BinOp::Assignment,
                rhs,
            } => {
                let Expr::Atom(frontend::ast::Atom::Id(id)) = lhs.as_ref() else {
                    return Err(Error::expected_lvalue());
                };

                let result = Id::from(id.clone());
                let value = rhs.translate(translator)?;

                translator.code.push(Copy { result, value });
            }

            frontend::ast::Expr::Call(call) => call.translate(translator)?,

            _ => {}
        }

        Ok(())
    }
}

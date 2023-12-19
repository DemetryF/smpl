mod atom;
mod call;

use smplc_ast::{BinOp, Expr};

use crate::{
    error::Error,
    instruction::{Atom, Binary, Unary},
    Translator,
};

use super::Translate;

impl Translate<Atom> for Expr {
    fn translate(self, translator: &mut Translator) -> Result<Atom, Error> {
        match self {
            Expr::Prefix { op, rhs } => {
                let result = translator.create_temp_variable();

                let rhs = rhs.translate(translator)?;

                translator.code.push(Unary {
                    result: result.clone(),
                    op,
                    rhs,
                });

                Ok(Atom::from(result))
            }

            Expr::Infix { lhs, op, rhs } => {
                if op == BinOp::Assign {
                    return Err(Error::unexpected_assignment());
                }

                let result = translator.create_temp_variable();

                let lhs = lhs.translate(translator)?;
                let rhs = rhs.translate(translator)?;

                translator.code.push(Binary {
                    result: result.clone(),
                    lhs,
                    op,
                    rhs,
                });

                Ok(Atom::from(result))
            }

            Expr::Call(call) => call.translate(translator),
            Expr::Atom(atom) => atom.translate(translator),
        }
    }
}

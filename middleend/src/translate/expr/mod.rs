mod atom;
mod call;

use smplc_ast::{BinOp, Expr};
use smplc_ir::{Atom, Binary, Unary};

use crate::{error::Error, Translator};

use super::Translate;

impl<'source> Translate<'source, Atom> for Expr<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<Atom, Error<'source>> {
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

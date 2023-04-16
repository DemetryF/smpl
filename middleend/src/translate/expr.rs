use frontend::ast::{BinOp, Expr};

use crate::{
    error::{Error, ErrorKind},
    instruction::{Atom, Binary, Call, Copy, Id, Push, Unary},
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

            Expr::Infix {
                lhs,
                op: BinOp::Assignment,
                rhs,
            } => {
                let value = rhs.translate(translator)?;

                let Expr::Atom(frontend::ast::Atom::Id(result)) = lhs.as_ref() else {
                    unreachable!();
                };

                let result = translator.scopes.add_variable(result.clone())?;

                translator.code.push(Copy {
                    result: result.clone(),
                    value,
                });

                Ok(Atom::from(result))
            }

            Expr::Infix { lhs, op, rhs } => {
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

            Expr::Call { id, args } => {
                let function = translator.scopes.get_function(&id)?;

                if function.args_count != args.len() {
                    let kind = ErrorKind::InvalidArgumentsCount {
                        expected_args_count: function.args_count,
                        received_args_count: args.len(),
                        function_id: id.clone(),
                    };

                    let error = Error::new(kind, id.pos);

                    translator.errors.push(error);
                }

                let result = translator.create_temp_variable();

                for arg in args {
                    let value = arg.translate(translator)?;

                    translator.code.push(Push { value });
                }

                let id = Id::from(id);

                translator.code.push(Call {
                    result: result.clone(),
                    id,
                });

                Ok(Atom::from(result))
            }

            Expr::Atom(atom) => match atom {
                frontend::ast::Atom::Id(id) => translator
                    .scopes
                    .get_variable(id)
                    .map(|variable| variable.id.into()),

                frontend::ast::Atom::Literal(literal) => Ok(Atom::from(literal)),
            },
        }
    }
}

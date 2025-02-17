use smplc_ast as ast;
use smplc_hir::{Atom, Expr};

use crate::env::Env;
use crate::error::{SemError, SemResult};
use crate::SemCheck;

impl<'source> SemCheck<'source> for ast::Expr<'source> {
    type Checked = Expr<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::Expr::Infix { lhs, op, rhs } => {
                let lhs = Box::new(lhs.0.check(env)?);
                let rhs = Box::new(rhs.0.check(env)?);

                Ok(Expr::Binary { lhs, op, rhs })
            }

            ast::Expr::Prefix { op, rhs } => {
                let rhs = Box::new(rhs.0.check(env)?);

                Ok(Expr::Unary { op, rhs })
            }

            ast::Expr::Call(call) => {
                let fun_ref = env.functions.get(call.id)?;

                {
                    let expected = fun_ref.args_types.len();
                    let received = call.args.len();

                    if expected != received {
                        return Err(SemError::invalid_arguments_count(
                            call.id.span(),
                            expected,
                            received,
                            fun_ref,
                        ));
                    }
                }

                let args = call
                    .args
                    .into_iter()
                    .map(|arg| arg.0.check(env))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Expr::Call { fun_ref, args })
            }

            ast::Expr::Atom(atom) => Ok(Expr::Atom(match atom {
                ast::Atom::Id(id) => Atom::Var(env.variables.get(id)?),
                ast::Atom::Literal(literal) => Atom::Literal(literal),
            })),
        }
    }
}

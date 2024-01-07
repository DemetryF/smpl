use smplc_ast as ast;

use smplc_hir::{Atom, Expr};

use super::SemCheck;
use crate::env::Env;
use crate::error::{SemError, SemResult};

impl<'source> SemCheck<'source> for ast::Expr<'source> {
    type Checked = Expr;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::Expr::Infix { lhs, op, rhs } => Ok(Expr::Binary {
                lhs: Box::new(lhs.check(env)?),
                op,
                rhs: Box::new(rhs.check(env)?),
            }),

            ast::Expr::Prefix { op, rhs } => Ok(Expr::Unary {
                op,
                rhs: Box::new(rhs.check(env)?),
            }),

            ast::Expr::Call(call) => {
                let fun_ref = env.functions.get(call.id)?;

                {
                    let expected = fun_ref.args_count;
                    let received = call.args.len();

                    if expected != received {
                        return Err(SemError::invalid_arguments(
                            call.id.pos,
                            expected,
                            received,
                            fun_ref,
                        ));
                    }
                }

                let args = call
                    .args
                    .into_iter()
                    .map(|arg| arg.check(env))
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

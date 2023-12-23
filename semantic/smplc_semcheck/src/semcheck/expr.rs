use smplc_ast as ast;

use smplc_hir::{Atom, Expr};

use super::SemCheck;
use crate::env::Env;
use crate::error::{SemError, SemResult};

impl<'source> SemCheck<'source> for ast::Expr<'source> {
    type Checked = Expr;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::Expr::Prefix { op, rhs } => Ok(Expr::Unary {
                op,
                rhs: Box::new(rhs.check(env)?),
            }),

            ast::Expr::Infix { lhs, op, rhs } => Ok(Expr::Binary {
                lhs: Box::new(lhs.check(env)?),
                op,
                rhs: Box::new(rhs.check(env)?),
            }),

            ast::Expr::Call(call) => {
                let function = env.functions.get(call.id)?;

                if call.args.len() != function.args_count {
                    return Err(SemError::invalid_arguments(
                        call.id.pos,
                        function.args_count,
                        call.args.len(),
                        function,
                    ));
                }

                let args = call
                    .args
                    .into_iter()
                    .map(|arg| arg.check(env))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Expr::Call { function, args })
            }

            ast::Expr::Atom(atom) => Ok(Expr::Atom(match atom {
                ast::Atom::Id(id) => Atom::Var(env.variables.get(id)?),
                ast::Atom::Literal(literal) => match literal {
                    ast::Literal::Number(num) => Atom::Value(num),
                    ast::Literal::Bool(bool) => Atom::Value(bool as u8 as f32),
                },
            })),
        }
    }
}

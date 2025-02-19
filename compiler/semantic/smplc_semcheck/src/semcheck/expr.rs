use smplc_ast as ast;
use smplc_ast::Call;
use smplc_hir::{Atom, Expr, FunData};

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
                let fun_id = env.functions.get(call.id)?;
                let fun_data = &env.functions.symbols[fun_id];

                check_args_count(fun_data, &call)?;

                let args = call
                    .args
                    .into_iter()
                    .map(|arg| arg.0.check(env))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Expr::Call { fun: fun_id, args })
            }

            ast::Expr::Atom(atom) => Ok(Expr::Atom(match atom {
                ast::Atom::Id(id) => Atom::Var(env.variables.get(id)?),
                ast::Atom::Literal(literal) => Atom::Literal(literal),
            })),
        }
    }
}

pub fn check_args_count<'source>(data: &FunData, call: &Call<'source>) -> SemResult<'source, ()> {
    let expected = data.args_types.len();
    let received = call.args.len();

    if expected != received {
        return Err(SemError::invalid_arguments_count(
            call.id, expected, received,
        ));
    }

    Ok(())
}

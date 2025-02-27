use smplc_ast as ast;
use smplc_ast::{Call, Spanned};
use smplc_hir::{Atom, Expr, FunData};

use crate::env::Env;
use crate::error::{SemError, SemResult};
use crate::inited::Inited;
use crate::SemCheck;

impl<'source> SemCheck<'source> for Spanned<ast::Expr<'source>> {
    type Checked = Spanned<Expr<'source>>;

    fn check(
        self,
        env: &mut Env<'source>,
        inited: &mut impl Inited,
    ) -> SemResult<'source, Self::Checked> {
        self.map(|expr| expr.check(env, inited)).transpose()
    }
}

impl<'source> SemCheck<'source> for ast::Expr<'source> {
    type Checked = Expr<'source>;

    fn check(
        self,
        env: &mut Env<'source>,
        inited: &mut impl Inited,
    ) -> SemResult<'source, Self::Checked> {
        match self {
            ast::Expr::Infix { lhs, op, rhs } => {
                let lhs = Box::new(lhs.check(env, inited)?);
                let rhs = Box::new(rhs.check(env, inited)?);

                Ok(Expr::Binary { lhs, op, rhs })
            }

            ast::Expr::Prefix { op, rhs } => {
                let rhs = Box::new(rhs.check(env, inited)?);

                Ok(Expr::Unary { op, rhs })
            }

            ast::Expr::Call(call) => {
                let fun = env.functions.get(call.id)?;
                let fun_data = &env.functions.symbols[fun];

                check_args_count(fun_data, &call)?;

                let args = call
                    .args
                    .into_iter()
                    .map(|arg| arg.check(env, inited))
                    .collect::<Result<_, _>>()?;

                Ok(Expr::Call { fun, args })
            }

            ast::Expr::Atom(atom) => Ok(Expr::Atom(match atom {
                ast::Atom::Id(id) => {
                    let var = env.variables.get(id)?;

                    if !inited.is_inited(var) {
                        return Err(SemError::using_uninited(id));
                    }

                    Atom::Var(var)
                }

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

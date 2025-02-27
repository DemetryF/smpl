use smplc_ast::{ConstantDeclaration, FunctionDeclaration};
use smplc_hir::{Constant, Function};

use crate::env::Env;
use crate::error::SemResult;
use crate::inited::Inited;

use super::SemCheck;

impl<'source> SemCheck<'source> for FunctionDeclaration<'source> {
    type Checked = Function<'source>;

    fn check(
        self,
        env: &mut Env<'source>,
        inited: &mut impl Inited,
    ) -> SemResult<'source, Self::Checked> {
        let id = env.functions.get(self.id).unwrap();

        env.variables.fork();

        let args = self
            .args
            .into_iter()
            .map(|id| env.variables.add_argument(id))
            .inspect(|maybe_var| {
                if let &Ok(var) = maybe_var {
                    inited.init(var);
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let body = self.body.check(env, inited)?;

        env.variables.exit();

        Ok(Function { id, args, body })
    }
}

impl<'source> SemCheck<'source> for ConstantDeclaration<'source> {
    type Checked = Constant<'source>;

    fn check(
        self,
        env: &mut Env<'source>,
        inited: &mut impl Inited,
    ) -> SemResult<'source, Self::Checked> {
        let id = env.variables.add_variable(self.id, Some(self.ty))?;

        inited.init(id);

        let value = self.value.0.check(env, inited)?;

        Ok(Constant {
            id,
            ty: self.ty,
            value,
        })
    }
}

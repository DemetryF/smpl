use smplc_ast::{ConstantDeclaration, FunctionDeclaration};
use smplc_hir::{Constant, Function};

use crate::env::Env;
use crate::error::SemResult;

use super::expr::expect_ty;
use super::SemCheck;

impl<'source> SemCheck<'source> for FunctionDeclaration<'source> {
    type Checked = Function<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let data = env.functions.get(self.id).unwrap();

        env.variables.fork();

        let args = self
            .args
            .into_iter()
            .map(|id| env.variables.add_argument(id))
            .collect::<Result<Vec<_>, _>>()?;

        let body = self.body.check(env)?.statements;

        env.variables.exit();

        Ok(Function { data, args, body })
    }
}

impl<'source> SemCheck<'source> for ConstantDeclaration<'source> {
    type Checked = Constant<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let data = env.variables.add_variable(self.id, self.ty)?;

        let span = self.value.span();
        let value = self.value.0.check(env)?;

        expect_ty(&value, data.ty, span)?;

        Ok(Constant { data, value })
    }
}

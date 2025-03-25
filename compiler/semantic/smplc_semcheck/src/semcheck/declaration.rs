use smplc_ast::{ConstantDeclaration, FunctionDeclaration};
use smplc_hir::{Constant, Function};

use crate::{env::Env, error::SemResult, inited::Inited};

use super::{RawType, SemCheck};

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
            .collect::<Result<_, _>>()?;

        for &arg in &args {
            inited.init(arg);
        }

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
        let ty = RawType(self.ty).checked()?;
        let id = env.variables.add_variable(self.id, Some(ty))?;
        let value = self.value.check(env, inited)?;

        inited.init(id);

        Ok(Constant { id, ty, value })
    }
}

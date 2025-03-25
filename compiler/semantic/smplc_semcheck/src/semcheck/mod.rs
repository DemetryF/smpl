mod declaration;
mod expr;
mod statement;
mod ty;

use smplc_ast as ast;
use smplc_hir::Block;

use crate::{
    error::SemResult,
    inited::{GeneralInited, Inited},
    Env,
};

pub use ty::RawType;

pub trait SemCheck<'source>: Sized {
    type Checked;

    fn check(
        self,
        env: &mut Env<'source>,
        inited: &mut impl Inited,
    ) -> SemResult<'source, Self::Checked>;
}

impl<'source> SemCheck<'source> for ast::Block<'source> {
    type Checked = Block<'source>;

    fn check(
        self,
        env: &mut Env<'source>,
        inited: &mut impl Inited,
    ) -> SemResult<'source, Self::Checked> {
        env.variables.fork();

        let mut inited = GeneralInited::with_parent(inited);

        let statements = self
            .statements
            .into_iter()
            .map(|stmt| stmt.check(env, &mut inited))
            .filter_map(Result::transpose)
            .collect::<Result<_, _>>()?;

        env.variables.exit();
        inited.exit();

        Ok(Block { statements })
    }
}

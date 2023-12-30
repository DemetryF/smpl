mod declaration;
mod expr;
mod statement;

use smplc_hir::Block;

use crate::error::SemResult;
use crate::Env;

pub trait SemCheck<'source>: Sized {
    type Checked;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked>;
}

impl<'source> SemCheck<'source> for smplc_ast::Block<'source> {
    type Checked = Block;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        env.variables.fork();

        let statements = self
            .stmts
            .into_iter()
            .map(|stmt| stmt.check(env))
            .collect::<Result<_, _>>()?;

        env.variables.exit();

        Ok(Block { statements })
    }
}

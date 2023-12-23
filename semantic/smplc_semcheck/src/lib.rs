use env::Env;
use error::SemResult;

pub mod error;

mod env;

pub trait SemCheck: Sized {
    type Checked;

    fn semcheck<'source>(self, env: Env<'source>) -> SemResult<'source, Self::Checked>;
}

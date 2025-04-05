use smplc_ast::Id;
use smplc_hir::Type;

use crate::error::{SemError, SemResult};

pub struct RawType<'source>(pub Id<'source>);

impl<'source> RawType<'source> {
    pub fn checked(self) -> SemResult<'source, Type> {
        Type::try_from(self.0 .0).map_err(|_| SemError::unknown_type(self.0))
    }
}

use frontend::ast::Prefix;

use crate::{Check, SharedEnv, StaticAnalyzer};

impl Check for Prefix {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        self.rhs.check(analyzer, env);
    }
}

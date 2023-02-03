use crate::{
    ast::Unary,
    static_analyzer::{check::Check, env::*, StaticAnalyzer},
};

impl Check for Unary {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        self.rhs.check(analyzer, env);
    }
}

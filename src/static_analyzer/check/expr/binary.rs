use crate::{
    ast::Binary,
    static_analyzer::{check::Check, env::Env, StaticAnalyzer},
};

impl Check for Binary {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        self.lhs.check(analyzer, env);
        self.rhs.check(analyzer, env);
    }
}

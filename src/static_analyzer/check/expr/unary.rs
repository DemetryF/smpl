use crate::{
    parser::ast::expr::unary::Unary,
    static_analyzer::{check::Check, env::Env, StaticAnalyzer},
};

impl Check for Unary {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        self.rhs.check(analyzer, env);
    }
}

use crate::{
    ast::ReturnStatement,
    static_analyzer::{check::Check, env::*, StaticAnalyzer},
};

impl Check for ReturnStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        if let Some(expr) = &self.0 {
            expr.check(analyzer, env);
        }
    }
}

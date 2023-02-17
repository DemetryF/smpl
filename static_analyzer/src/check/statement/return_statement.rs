use frontend::ast::ReturnStatement;

use crate::{Check, SharedEnv, StaticAnalyzer};

impl Check for ReturnStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        if let Some(expr) = &self.0 {
            expr.check(analyzer, env);
        }
    }
}

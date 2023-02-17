use std::rc::Rc;

use frontend::ast::WhileStatement;

use crate::{Check, SharedEnv, StaticAnalyzer};

impl Check for WhileStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        self.cond.check(analyzer, Rc::clone(&env));
        self.body.check(analyzer, Rc::clone(&env));
    }
}

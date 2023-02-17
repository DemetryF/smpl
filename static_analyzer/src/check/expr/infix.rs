use std::rc::Rc;

use frontend::ast::Infix;

use crate::{Check, SharedEnv, StaticAnalyzer};

impl Check for Infix {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        self.lhs.check(analyzer, Rc::clone(&env));
        self.rhs.check(analyzer, Rc::clone(&env));
    }
}

use std::rc::Rc;

use crate::{
    ast::WhileStatement,
    static_analyzer::{check::Check, env::*, StaticAnalyzer},
};

impl Check for WhileStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        self.cond.check(analyzer, Rc::clone(&env));
        self.body.check(analyzer, Rc::clone(&env));
    }
}

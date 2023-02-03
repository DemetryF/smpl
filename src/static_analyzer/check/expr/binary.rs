use std::rc::Rc;

use crate::{
    ast::Binary,
    static_analyzer::{check::Check, env::*, StaticAnalyzer},
};

impl Check for Binary {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        self.lhs.check(analyzer, Rc::clone(&env));
        self.rhs.check(analyzer, Rc::clone(&env));
    }
}

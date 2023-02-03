use std::rc::Rc;

use crate::{
    ast::IfStatement,
    static_analyzer::{check::Check, env::*, StaticAnalyzer},
};

impl Check for IfStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        self.cond.check(analyzer, Rc::clone(&env));
        self.then_body.check(analyzer, Rc::clone(&env));

        if let Some(else_body) = &self.else_body {
            else_body.check(analyzer, Rc::clone(&env));
        }
    }
}

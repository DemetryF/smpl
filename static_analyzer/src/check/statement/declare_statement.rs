use std::rc::Rc;

use frontend::ast::DeclareStatement;

use crate::{Check, SharedEnv, StaticAnalyzer};

impl Check for DeclareStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        let mut borrowed_env = env.borrow_mut();

        match borrowed_env.variables.get(&self.id.value) {
            None => borrowed_env.add_variable(self.id.clone()),
            Some(id) => analyzer.redeclaring_error(self.id.clone(), id),
        }

        if let Some(expr) = &self.expr {
            expr.check(analyzer, Rc::clone(&env));
        }
    }
}

use frontend::ast::Atom;

use crate::{Check, SharedEnv, StaticAnalyzer};

impl Check for Atom {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        if let Atom::Id(id) = &self {
            if !env.borrow_mut().search(&id.value) {
                analyzer.non_existing_variable_error(id.to_owned());
            }
        }
    }
}

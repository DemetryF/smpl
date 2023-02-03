use crate::{
    ast::Atom,
    static_analyzer::{check::Check, env::*, StaticAnalyzer},
};

impl Check for Atom {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        if let Atom::Id(id) = &self {
            if !env.borrow_mut().search(&id.value) {
                analyzer.non_existing_variable_error(id.to_owned());
            }
        }
    }
}

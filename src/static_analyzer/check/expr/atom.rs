use crate::{
    parser::ast::expr::Atom,
    static_analyzer::{ check::Check, env::Env, StaticAnalyzer},
};

impl Check for Atom {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        if let Atom::Id(id) = &self {
            if !env.search(&id.value) {
                analyzer.non_existing_variable_error(id.to_owned());
            }
        }
    }
}

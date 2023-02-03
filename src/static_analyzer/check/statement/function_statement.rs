use std::cell::RefCell;

use itertools::Itertools;

use crate::{
    ast::FunctionStatement,
    static_analyzer::{check::Check, env::*, StaticAnalyzer},
};

impl Check for FunctionStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, _: SharedEnv) {
        self.check_args(analyzer);

        analyzer.add_function(self.id.clone(), self.args.len());

        let env = self.env_from_args();

        self.body.check(analyzer, SharedEnv::new(RefCell::new(env)));
    }
}

impl FunctionStatement {
    fn check_args(&self, analyzer: &mut StaticAnalyzer) {
        let mut duplicates = self.args.iter().duplicates_by(|x| &x.value);

        match duplicates.next() {
            None => (),
            Some(id) => analyzer.duplicate_args_error(id.to_owned()),
        }
    }

    fn env_from_args(&self) -> Env {
        let mut env = Env::new();

        for arg in self.args.iter() {
            env.add_variable(arg.to_owned());
        }

        env
    }
}

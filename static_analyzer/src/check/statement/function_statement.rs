use std::cell::RefCell;

use itertools::Itertools;

use frontend::ast::{FunctionStatement, Id};

use crate::{Check, Env, SharedEnv, StaticAnalyzer};

impl Check for FunctionStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, _: SharedEnv) {
        check_args(self, analyzer);

        analyzer.add_function(self.id.clone(), self.args.len());

        let env = env_from_args(&self.args);

        self.body.check(analyzer, SharedEnv::new(RefCell::new(env)));
    }
}

fn check_args(function: &FunctionStatement, analyzer: &mut StaticAnalyzer) {
    let mut duplicates = function.args.iter().duplicates_by(|x| &x.value);

    match duplicates.next() {
        None => (),
        Some(id) => analyzer.duplicate_args_error(id.to_owned()),
    }
}

fn env_from_args(args: &Vec<Id>) -> Env {
    let mut env = Env::new();

    for arg in args {
        env.add_variable(arg.to_owned());
    }

    env
}

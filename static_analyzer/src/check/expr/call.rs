use std::rc::Rc;

use frontend::ast::Call;

use crate::{Check, SharedEnv, StaticAnalyzer};

impl Check for Call {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        match analyzer.functions.get_mut(&self.id.value) {
            Some(_) => check_args_count(self, analyzer),
            None => analyzer.non_existing_function_error(self.id.to_owned()),
        }

        check_args(self, analyzer, env);
    }
}

fn check_args(call: &Call, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
    for arg in call.args.iter() {
        arg.check(analyzer, Rc::clone(&env));
    }
}

fn check_args_count(call: &Call, analyzer: &mut StaticAnalyzer) {
    let func = analyzer.functions.get_mut(&call.id.value).unwrap();

    if func.args_count != call.args.len() {
        func.uses_count += 1;
        analyzer.invalid_args_count_error(call);
    }
}

use std::rc::Rc;

use crate::{
    ast::Call,
    static_analyzer::{check::Check, env::*, StaticAnalyzer},
};

impl Check for Call {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        match analyzer.functions.get_mut(&self.id.value) {
            Some(_) => self.check_args_count(analyzer),
            None => analyzer.non_existing_function_error(self.id.to_owned()),
        }

        self.check_args(analyzer, env);
    }
}

impl Call {
    fn check_args(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        for arg in self.args.iter() {
            arg.check(analyzer, Rc::clone(&env));
        }
    }

    fn check_args_count(&self, analyzer: &mut StaticAnalyzer) {
        let func = analyzer.functions.get_mut(&self.id.value).unwrap();

        if func.args_count != self.args.len() {
            func.uses_count += 1;
            analyzer.invalid_args_count_error(self);
        }
    }
}

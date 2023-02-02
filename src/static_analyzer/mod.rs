use derive_more::Constructor;
use std::collections::HashMap;

use self::check::Check;
use crate::{
    ast::{Call, Id, Statement},
    error::*,
};

pub use env::{Env, StaticIdInfo};

mod check;
pub mod env;

#[derive(Constructor)]
pub struct StaticFunctionInfo {
    uses_count: usize,
    args_count: usize,
    id: Id,
}

pub struct StaticAnalyzer {
    pub errors: Vec<Error>,
    pub functions: HashMap<String, StaticFunctionInfo>,
}

impl StaticAnalyzer {
    pub fn new(all_stmts: &Vec<Statement>) -> Self {
        let mut global_env = Env::new();

        let mut analyzer = Self {
            errors: Vec::new(),
            functions: HashMap::new(),
        };

        let (funcs, stmts) = Self::sort(all_stmts);

        for func in funcs {
            func.check(&mut analyzer, &mut global_env);
        }

        for stmt in stmts {
            stmt.check(&mut analyzer, &mut global_env)
        }

        analyzer
    }

    pub fn sort(all_stmts: &Vec<Statement>) -> (Vec<&Statement>, Vec<&Statement>) {
        let mut funcs = Vec::new();
        let mut stmts = Vec::new();

        for stmt in all_stmts {
            match stmt {
                Statement::Function(_) => funcs.push(stmt),
                stmt => stmts.push(stmt),
            }
        }

        (funcs, stmts)
    }

    pub fn redeclaring_error(&mut self, id: Id, existing_id: &StaticIdInfo) {
        self.errors.push(Error::new(
            ErrorKind::ReDeclaringVariable {
                name: id.value,
                defined_at: existing_id.define_pos,
            },
            id.pos,
        ));
    }

    pub fn add_function(&mut self, id: Id, args_count: usize) {
        self.functions.insert(
            id.value.clone(),
            StaticFunctionInfo {
                uses_count: 0,
                args_count,
                id,
            },
        );
    }

    pub fn duplicate_args_error(&mut self, id: Id) {
        self.errors.push(Error::new(
            ErrorKind::DuplicateFunctionArgs(id.value),
            id.pos,
        ))
    }

    pub fn non_existing_variable_error(&mut self, id: Id) {
        self.errors
            .push(Error::new(ErrorKind::NonExistingVariable(id.value), id.pos));
    }

    pub fn non_existing_function_error(&mut self, id: Id) {
        self.errors
            .push(Error::new(ErrorKind::NonExistingFunction(id.value), id.pos))
    }

    pub fn invalid_args_count_error(&mut self, call: &Call) {
        let func = self.functions.get_mut(&call.id.value).unwrap();

        self.errors.push(Error::new(
            ErrorKind::InvalidArgumentsCount {
                expected_args_count: func.args_count,
                received_args_count: call.args.len(),
                function_id: func.id.clone(),
            },
            call.id.pos,
        ))
    }
}

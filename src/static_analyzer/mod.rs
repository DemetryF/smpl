use std::collections::HashMap;

use derive_more::Constructor;

use crate::parser::ast::{statement::Statement, Id};

use self::{check::Check, env::Env, static_error::StaticError};

pub mod check;
pub mod env;
pub mod static_error;

#[derive(Constructor)]
pub struct StaticFunctionInfo {
    uses_count: usize,
    args_count: usize,
    id: Id,
}

pub struct StaticAnalyzer {
    pub errors: Vec<StaticError>,
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
}

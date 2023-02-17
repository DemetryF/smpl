use frontend::ast::Statement;

use crate::{Check, SharedEnv, StaticAnalyzer};

pub mod declare_statement;
pub mod function_statement;
pub mod if_statement;
pub mod return_statement;
pub mod while_statement;

impl Check for Statement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        match self {
            Statement::Expr(expr) => expr.check(analyzer, env),
            Statement::Declare(declare) => declare.check(analyzer, env),
            Statement::If(if_stmt) => if_stmt.check(analyzer, env),
            Statement::Function(function) => function.check(analyzer, env),
            Statement::Return(return_statement) => return_statement.check(analyzer, env),
            Statement::While(while_statement) => while_statement.check(analyzer, env),
        }
    }
}

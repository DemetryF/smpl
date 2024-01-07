use smplc_ast as ast;
use smplc_hir::{Atom, ExprStatement, IfStatement, ReturnStatement, Statement, WhileStatement};

use super::SemCheck;
use crate::env::Env;
use crate::error::SemResult;

impl<'source> SemCheck<'source> for ast::Statement<'source> {
    type Checked = Statement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::Statement::Declare(declare_stmt) => declare_stmt.check(env).map(Statement::Expr),
            ast::Statement::If(if_stmt) => if_stmt.check(env).map(Statement::If),
            ast::Statement::While(while_stmt) => while_stmt.check(env).map(Statement::While),
            ast::Statement::Expr(expr_stmt) => expr_stmt.check(env).map(Statement::Expr),
            ast::Statement::Return(return_stmt) => return_stmt.check(env).map(Statement::Return),

            ast::Statement::Break => Ok(Statement::Break),
            ast::Statement::Continue => Ok(Statement::Continue),
        }
    }
}

impl<'source> SemCheck<'source> for ast::DeclareStatement<'source> {
    type Checked = ExprStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let to = env.variables.add_variable(self.id)?;

        let what = self.value.map(|expr| expr.check(env)).transpose()?;
        let what = what.unwrap_or(smplc_hir::Expr::Atom(Atom::Value(0.0)));

        Ok(ExprStatement::Assign { var: to, rhs: what })
    }
}

impl<'source> SemCheck<'source> for ast::ExprStatement<'source> {
    type Checked = ExprStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::ExprStatement::Expr(expr) => expr.check(env).map(ExprStatement::Expr),
            ast::ExprStatement::Assign { id, rhs: expr } => Ok(ExprStatement::Assign {
                var: env.variables.get(id)?,
                rhs: expr.check(env)?,
            }),
        }
    }
}

impl<'source> SemCheck<'source> for ast::IfStatement<'source> {
    type Checked = IfStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let cond = self.cond.check(env)?;
        let then_body = self.body.check(env)?;
        let else_body = self.else_body.map(|body| body.check(env)).transpose()?;

        Ok(IfStatement {
            cond,
            body: then_body,
            else_body,
        })
    }
}

impl<'source> SemCheck<'source> for ast::ReturnStatement<'source> {
    type Checked = ReturnStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let value = self.value.map(|expr| expr.check(env)).transpose()?;

        Ok(ReturnStatement { value })
    }
}

impl<'source> SemCheck<'source> for ast::WhileStatement<'source> {
    type Checked = WhileStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let cond = self.cond.check(env)?;
        let body = self.body.check(env)?;

        Ok(WhileStatement { cond, body })
    }
}

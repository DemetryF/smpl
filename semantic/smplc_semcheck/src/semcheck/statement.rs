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

            ast::Statement::Function(_) => unreachable!(),
        }
    }
}

impl<'source> SemCheck<'source> for ast::DeclareStatement<'source> {
    type Checked = ExprStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let to = env.variables.add_variable(self.id)?;

        let what = self.init_expr.map(|expr| expr.check(env)).transpose()?;
        let what = what.unwrap_or(smplc_hir::Expr::Atom(Atom::Value(0.0)));

        Ok(ExprStatement::Assign { to, what })
    }
}

impl<'source> SemCheck<'source> for ast::ExprStatement<'source> {
    type Checked = ExprStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::ExprStatement::Expr(expr) => expr.check(env).map(ExprStatement::Expr),
            ast::ExprStatement::Assign { id, expr } => Ok(ExprStatement::Assign {
                to: env.variables.get(id)?,
                what: expr.check(env)?,
            }),
        }
    }
}

impl<'source> SemCheck<'source> for ast::IfStatement<'source> {
    type Checked = IfStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let cond = self.condition.check(env)?;
        let then_body = self.then_body.check(env)?;
        let else_body = self.else_body.map(|body| body.check(env)).transpose()?;

        Ok(IfStatement {
            cond,
            then_body,
            else_body,
        })
    }
}

impl<'source> SemCheck<'source> for ast::ReturnStatement<'source> {
    type Checked = ReturnStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        Ok(ReturnStatement {
            expr: self.0.map(|expr| expr.check(env)).transpose()?,
        })
    }
}

impl<'source> SemCheck<'source> for ast::WhileStatement<'source> {
    type Checked = WhileStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let cond = self.condition.check(env)?;
        let body = self.body.check(env)?;

        Ok(WhileStatement { cond, body })
    }
}

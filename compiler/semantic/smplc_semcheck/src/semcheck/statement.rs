use smplc_ast as ast;
use smplc_hir::*;

use crate::env::Env;
use crate::error::SemResult;

use super::SemCheck;

impl<'source> SemCheck<'source> for ast::Statement<'source> {
    type Checked = Option<Statement<'source>>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::Statement::Declare(declare_stmt) => {
                return Ok(declare_stmt.check(env)?.map(Statement::Expr));
            }

            ast::Statement::If(if_stmt) => {
                return Ok(Some(Statement::If(if_stmt.check(env)?)));
            }

            ast::Statement::While(while_stmt) => {
                return Ok(Some(Statement::While(while_stmt.check(env)?)));
            }

            ast::Statement::Expr(expr_stmt) => {
                return Ok(Some(Statement::Expr(expr_stmt.check(env)?)));
            }

            ast::Statement::Return(return_stmt) => {
                return Ok(Some(Statement::Return(return_stmt.check(env)?)));
            }

            ast::Statement::Break => Ok(Some(Statement::Break)),
            ast::Statement::Continue => Ok(Some(Statement::Continue)),
        }
    }
}

impl<'source> SemCheck<'source> for ast::DeclareStatement<'source> {
    type Checked = Option<ExprStatement<'source>>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let var = env.variables.add_variable(self.id, self.ty)?;

        Ok(self
            .value
            .map(|expr| expr.check(env))
            .transpose()?
            .map(|rhs| ExprStatement::Assign { var, rhs }))
    }
}

impl<'source> SemCheck<'source> for ast::ExprStatement<'source> {
    type Checked = ExprStatement<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::ExprStatement::Expr(expr) => {
                return Ok(ExprStatement::Expr(expr.0.check(env)?));
            }

            ast::ExprStatement::Assign { id, rhs } => {
                let var = env.variables.get(id)?;
                let rhs = rhs.check(env)?;

                Ok(ExprStatement::Assign { var, rhs })
            }
        }
    }
}

impl<'source> SemCheck<'source> for ast::IfStatement<'source> {
    type Checked = IfStatement<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let cond = self.cond.check(env)?;
        let body = self.body.check(env)?;

        let else_body = self.else_body.map(|body| body.check(env)).transpose()?;

        Ok(IfStatement {
            cond,
            body,
            else_body,
        })
    }
}

impl<'source> SemCheck<'source> for ast::ReturnStatement<'source> {
    type Checked = ReturnStatement<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let value = self.value.map(|expr| expr.check(env)).transpose()?;

        Ok(ReturnStatement { value })
    }
}

impl<'source> SemCheck<'source> for ast::WhileStatement<'source> {
    type Checked = WhileStatement<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let cond = self.cond.check(env)?;
        let body = self.body.check(env)?;

        Ok(WhileStatement { cond, body })
    }
}

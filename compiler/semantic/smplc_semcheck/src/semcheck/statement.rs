use smplc_ast as ast;
use smplc_hir::*;

use super::expr::expect_ty;
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
        let var = env.variables.add_variable(self.id, self.ty)?;

        let rhs = self.value.map(|expr| expr.check(env)).transpose()?;

        if let Some(expr) = &rhs {
            expect_ty(expr, var.ty)?;
        }

        let rhs = rhs.unwrap_or(Expr::Atom(Atom::Literal(match self.ty {
            Type::Real => ast::Literal::Real(0.0),
            Type::Int => ast::Literal::Int(0),
            Type::Bool => ast::Literal::Bool(false),
        })));

        Ok(ExprStatement::Assign { var, rhs })
    }
}

impl<'source> SemCheck<'source> for ast::ExprStatement<'source> {
    type Checked = ExprStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::ExprStatement::Expr(expr) => expr.check(env).map(ExprStatement::Expr),
            ast::ExprStatement::Assign { id, rhs: expr } => {
                let var = env.variables.get(id)?;
                let rhs = expr.check(env)?;

                expect_ty(&rhs, var.ty)?;

                Ok(ExprStatement::Assign { var, rhs })
            }
        }
    }
}

impl<'source> SemCheck<'source> for ast::IfStatement<'source> {
    type Checked = IfStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let cond = self.cond.check(env)?;

        expect_ty(&cond, Type::Bool)?;

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
    type Checked = ReturnStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let value = self.value.map(|expr| expr.check(env)).transpose()?;

        if let Some(_) = &value { /* add typecheck here */ }

        Ok(ReturnStatement { value })
    }
}

impl<'source> SemCheck<'source> for ast::WhileStatement<'source> {
    type Checked = WhileStatement;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let cond = self.cond.check(env)?;

        expect_ty(&cond, Type::Bool)?;

        let body = self.body.check(env)?;

        Ok(WhileStatement { cond, body })
    }
}

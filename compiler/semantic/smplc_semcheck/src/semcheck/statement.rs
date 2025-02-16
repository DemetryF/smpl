use smplc_ast as ast;
use smplc_hir::*;

use crate::env::Env;
use crate::error::{SemError, SemResult};

use super::expr::{expect_ty, expr_ty};
use super::SemCheck;

impl<'source> SemCheck<'source> for ast::Statement<'source> {
    type Checked = Statement<'source>;

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
    type Checked = ExprStatement<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let var = env.variables.add_variable(self.id, self.ty.unwrap())?;

        let rhs = {
            if let Some(ast::Spanned(rhs, span)) = self.value {
                let rhs = rhs.check(env)?;

                expect_ty(&rhs, var.ty, span)?;

                rhs
            } else {
                let value = match self.ty.unwrap() {
                    Type::Real => "0.0",
                    Type::Int => "0",
                    Type::Bool => "false",
                };

                Expr::Atom(Atom::Literal(Literal {
                    value,
                    ty: self.ty.unwrap(),
                }))
            }
        };

        Ok(ExprStatement::Assign { var, rhs })
    }
}

impl<'source> SemCheck<'source> for ast::ExprStatement<'source> {
    type Checked = ExprStatement<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::ExprStatement::Expr(expr) => expr.0.check(env).map(ExprStatement::Expr),

            ast::ExprStatement::Assign {
                id,
                rhs: ast::Spanned(rhs, span),
            } => {
                let var = env.variables.get(id)?;
                let rhs = rhs.check(env)?;

                expect_ty(&rhs, var.ty, span)?;

                Ok(ExprStatement::Assign { var, rhs })
            }
        }
    }
}

impl<'source> SemCheck<'source> for ast::IfStatement<'source> {
    type Checked = IfStatement<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let span = self.cond.span();
        let cond = self.cond.0.check(env)?;

        expect_ty(&cond, Type::Bool, span)?;

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
        let value = {
            if let Some(ast::Spanned(expr, span)) = self.value {
                let expr = expr.check(env)?;

                if let Some(ty) = env.current_fn.as_ref().unwrap().ret_ty {
                    expect_ty(&expr, ty, span)?;
                } else {
                    return Err(SemError::wrong_ty(span, expr_ty(&expr), vec![]));
                }

                Some(expr)
            } else {
                None
            }
        };

        Ok(ReturnStatement { value })
    }
}

impl<'source> SemCheck<'source> for ast::WhileStatement<'source> {
    type Checked = WhileStatement<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        let span = self.cond.span();
        let cond = self.cond.0.check(env)?;

        expect_ty(&cond, Type::Bool, span)?;

        let body = self.body.check(env)?;

        Ok(WhileStatement { cond, body })
    }
}

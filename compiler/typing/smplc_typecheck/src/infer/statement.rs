use smplc_hir as hir;
use smplc_hir::Type;

use crate::error::{TypeError, TypeResult};

use super::expr::{infer_expr, InferenceResult};
use super::{TypeInfer, TypeInferrer, TypeVar};

impl<'source> TypeInfer<'source> for hir::Statement<'source> {
    fn infer(
        &self,
        inferrer: &mut TypeInferrer,
        symbols: &hir::Symbols<'source>,
    ) -> TypeResult<'source, ()> {
        match self {
            hir::Statement::Expr(expr_statement) => expr_statement.infer(inferrer, symbols),
            hir::Statement::If(if_statement) => if_statement.infer(inferrer, symbols),
            hir::Statement::Return(return_statement) => return_statement.infer(inferrer, symbols),
            hir::Statement::While(while_statement) => while_statement.infer(inferrer, symbols),

            _ => Ok(()),
        }
    }
}

impl<'source> TypeInfer<'source> for hir::ExprStatement<'source> {
    fn infer(
        &self,
        inferrer: &mut TypeInferrer,
        symbols: &hir::Symbols<'source>,
    ) -> TypeResult<'source, ()> {
        match self {
            &hir::ExprStatement::Assign { var, ref rhs } => {
                let InferenceResult {
                    set: value_set,
                    ty: value_ty,
                    ..
                } = infer_expr(&rhs, inferrer, symbols)?;

                let var_set = if let Some(ty) = symbols.variables[var].ty {
                    TypeVar::max(TypeVar::Type(ty), value_ty).map_err(|(required, got)| {
                        TypeError::mismatched_types(required, got, rhs.span())
                    })?;

                    if let Some(set) = value_set {
                        inferrer.set_set_ty(set, TypeVar::Type(ty)).unwrap();
                    }

                    inferrer.set_var_ty(var, TypeVar::Type(ty)).unwrap()
                } else {
                    inferrer.set_var_ty(var, value_ty).unwrap()
                };

                if let Some(value_set) = value_set {
                    inferrer.unite(var_set, value_set).unwrap();
                }

                Ok(())
            }

            hir::ExprStatement::Expr(expr) => {
                infer_expr(expr, inferrer, symbols)?;

                Ok(())
            }
        }
    }
}

impl<'source> TypeInfer<'source> for hir::IfStatement<'source> {
    fn infer(
        &self,
        inferrer: &mut TypeInferrer,
        symbols: &hir::Symbols<'source>,
    ) -> TypeResult<'source, ()> {
        let InferenceResult { set, ty, .. } = infer_expr(&self.cond, inferrer, symbols)?;

        TypeVar::max(ty, TypeVar::Type(Type::Bool)).map_err(|(got, required)| {
            TypeError::mismatched_types(required, got, self.cond.span())
        })?;

        if let Some(set) = set {
            inferrer.set_set_ty(set, TypeVar::Type(Type::Bool)).unwrap();
        }

        self.body.infer(inferrer, symbols)?;

        if let Some(block) = &self.else_body {
            block.infer(inferrer, symbols)?;
        }

        Ok(())
    }
}

impl<'source> TypeInfer<'source> for hir::ReturnStatement<'source> {
    fn infer(
        &self,
        inferrer: &mut TypeInferrer,
        symbols: &hir::Symbols<'source>,
    ) -> TypeResult<'source, ()> {
        let ret_ty = symbols.functions[inferrer.current_fn.unwrap()].ret_ty;
        let ret_ty = ret_ty.map_or(TypeVar::None, TypeVar::Type);

        match &self.value {
            Some(value) => {
                let InferenceResult { set, ty, .. } = infer_expr(&value, inferrer, symbols)?;

                TypeVar::max(ty, ret_ty).map_err(|(got, required)| {
                    TypeError::mismatched_types(required, got, value.span())
                })?;

                if let Some(set) = set {
                    inferrer.set_set_ty(set, ret_ty).unwrap();
                }
            }

            None if ret_ty != TypeVar::None => {
                todo!("make Type Error")
            }

            _ => {}
        }

        Ok(())
    }
}

impl<'source> TypeInfer<'source> for hir::WhileStatement<'source> {
    fn infer(
        &self,
        inferrer: &mut TypeInferrer,
        symbols: &hir::Symbols<'source>,
    ) -> TypeResult<'source, ()> {
        let InferenceResult { set, ty, .. } = infer_expr(&self.cond, inferrer, symbols)?;

        TypeVar::max(ty, TypeVar::Type(Type::Bool)).map_err(|(got, required)| {
            TypeError::mismatched_types(required, got, self.cond.span())
        })?;

        if let Some(set) = set {
            inferrer.set_set_ty(set, TypeVar::Type(Type::Bool)).unwrap();
        }

        self.body.infer(inferrer, symbols)?;

        Ok(())
    }
}

impl<'source> TypeInfer<'source> for hir::Block<'source> {
    fn infer(
        &self,
        inferrer: &mut TypeInferrer,
        symbols: &hir::Symbols<'source>,
    ) -> TypeResult<'source, ()> {
        for stmt in &self.statements {
            stmt.infer(inferrer, symbols)?;
        }

        Ok(())
    }
}

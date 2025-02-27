use smplc_hir as hir;
use smplc_hir::Type;

use crate::error::{TypeError, TypeResult};

use super::{SetId, TypeInferrer, TypeVar};

pub fn infer_expr<'source>(
    expr: &hir::Expr<'source>,
    inferrer: &mut TypeInferrer,
    symbols: &hir::Symbols<'source>,
) -> TypeResult<'source, InferenceResult> {
    match expr {
        hir::Expr::Binary { lhs, op, rhs } => {
            let lhs_inference = infer_expr(&lhs.0, inferrer, symbols)?;
            let rhs_inference = infer_expr(&rhs.0, inferrer, symbols)?;

            let mut op_compability = {
                |op: &hir::BinOp, inference: InferenceResult, span| {
                    match op {
                        op if op.is_arithm() => TypeVar::max(inference.ty, TypeVar::Number),

                        op if op.is_logic() => {
                            TypeVar::max(inference.ty, TypeVar::Type(Type::Bool))
                        }

                        op if op.is_rel() => TypeVar::max(inference.ty, TypeVar::Number),
                        _ => unreachable!(),
                    }
                    .inspect(|&ty| {
                        if let Some(set) = inference.set {
                            inferrer.set_set_ty(set, ty).unwrap();
                        }
                    })
                    .map_err(|(got, required)| TypeError::mismatched_types(required, got, span))
                }
            };

            let lhs_ty = op_compability(op, lhs_inference, lhs.span())?;
            let rhs_ty = op_compability(op, rhs_inference, rhs.span())?;

            // calculate the operation type
            let operation_ty = {
                let maybe_ty = TypeVar::max(lhs_ty, rhs_ty);

                maybe_ty.map_err(|(required, got)| {
                    TypeError::mismatched_types(required, got, rhs.span())
                })?
            };

            // unite sets because all bin ops require same types
            let set = match (lhs_inference.set, rhs_inference.set) {
                (Some(a), Some(b)) => Some(inferrer.unite(a, b).unwrap()),
                (None, Some(set)) | (Some(set), None) => Some(set),
                (None, None) => None,
            };

            let result_ty = match op {
                op if op.is_rel() => TypeVar::Type(Type::Bool),
                _ => operation_ty,
            };

            if let Some(set) = set {
                inferrer.set_set_ty(set, operation_ty).unwrap();
            }

            // rel ops have different operands and result types
            let set = match op {
                op if op.is_rel() => None,
                _ => set,
            };

            Ok(InferenceResult { set, ty: result_ty })
        }

        hir::Expr::Unary { op, rhs } => {
            let InferenceResult {
                set,
                ty: operand_ty,
            } = infer_expr(&rhs.0, inferrer, symbols)?;

            let min_ty = match op {
                hir::UnOp::Not => TypeVar::Type(Type::Bool),
                hir::UnOp::Neg => TypeVar::Number,
            };

            let ret_ty = TypeVar::max(min_ty, operand_ty).map_err(|(required, got)| {
                TypeError::mismatched_types(required, got, rhs.span())
            })?;

            Ok(InferenceResult { set, ty: ret_ty })
        }

        hir::Expr::Call { fun, args } => {
            let &fun_id = fun;
            let fun = &symbols.functions[fun_id];

            for (expr, &req_ty) in args.iter().zip(&fun.args_types) {
                let InferenceResult { set, ty: arg_ty } = infer_expr(&expr.0, inferrer, symbols)?;

                if let Err((got, required)) = TypeVar::max(TypeVar::Type(req_ty), arg_ty) {
                    return Err(TypeError::mismatched_types(required, got, expr.span()));
                }

                if let Some(set) = set {
                    inferrer.set_set_ty(set, TypeVar::Type(req_ty)).unwrap();
                }
            }

            let ret_ty = fun.ret_ty.map(TypeVar::Type).unwrap_or(TypeVar::None);

            Ok(InferenceResult {
                set: None,
                ty: ret_ty,
            })
        }

        &hir::Expr::Atom(hir::Atom::Var(var)) => {
            if !inferrer.vars.contains_key(&var) {
                let ty = symbols.variables[var].ty;
                let ty = ty.map_or(TypeVar::Unknown, TypeVar::Type);

                inferrer.set_var_ty(var, ty).unwrap();
            }

            let set = inferrer.vars[&var];
            let ret_ty = inferrer.sets[&set];

            Ok(InferenceResult {
                set: Some(set),
                ty: ret_ty,
            })
        }

        hir::Expr::Atom(hir::Atom::Literal(lit)) => Ok(InferenceResult {
            set: None,
            ty: TypeVar::Type(lit.ty),
        }),
    }
}

#[derive(Clone, Copy)]
pub struct InferenceResult {
    pub set: Option<SetId>,
    pub ty: TypeVar,
}

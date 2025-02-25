use smplc_hir as hir;
use smplc_hir::Type;

use crate::error::TypeResult;

use super::{SetId, TypeInferrer, TypeVar};

pub fn infer_expr<'source>(
    expr: &hir::Expr<'source>,
    inferrer: &mut TypeInferrer,
    symbols: &hir::Symbols<'source>,
) -> TypeResult<'source, InferenceResult> {
    match expr {
        hir::Expr::Binary { lhs, op, rhs } => {
            // get inference information about lhs and rhs
            let lhs_inference = infer_expr(&lhs.0, inferrer, symbols)?;
            let rhs_inference = infer_expr(&rhs.0, inferrer, symbols)?;

            // calculate the most inferred type var of both
            let operands_ty = match TypeVar::max(lhs_inference.ty, rhs_inference.ty) {
                Ok(ty) => ty,
                Err(_) => todo!("make TypeError"),
            };

            // both expression anyway are linked and the op expects the same types
            // (there's only rel (a > 10), arithm (1 + a * c) and logic (a || b && true) binary ops)
            // so that, we link them into one set
            let set = match (lhs_inference.set, rhs_inference.set) {
                (None, None) => None,
                (None, Some(set)) | (Some(set), None) => Some(set),
                (Some(a), Some(b)) => Some(inferrer.unite(a, b).expect("make TypeError")),
            };

            // but rel ops don't link type of the operands with the caller
            // so we don't return the set
            let set = match op {
                op if op.is_rel() => None,
                _ => set,
            };

            // check the operands ty and the op compability
            // and calculate the ret ty
            let ret_ty = match op {
                op if op.is_arithm() => {
                    // if a arithm op is given, the ty must be Number, and we return it
                    TypeVar::max(TypeVar::Number, operands_ty).expect("make TypeError")
                }
                op if op.is_rel() => {
                    // if a rel op is given, the ret ty is bool, but operands' ty must be Number
                    TypeVar::max(TypeVar::Number, operands_ty).expect("make TypeError");

                    TypeVar::Type(Type::Bool)
                }

                op if op.is_logic() => {
                    // if a logic op is given, the ret ty is bool and operands' ty too must be bool
                    TypeVar::max(TypeVar::Type(Type::Bool), operands_ty).expect("make TypeError")
                }

                _ => unreachable!(),
            };

            Ok(InferenceResult { set, ty: ret_ty })
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

            let ret_ty = TypeVar::max(min_ty, operand_ty).expect("make TypeError");

            Ok(InferenceResult { set, ty: ret_ty })
        }

        hir::Expr::Call { fun, args } => {
            let &fun_id = fun;
            let fun = &symbols.functions[fun_id];

            for (expr, &req_ty) in args.iter().zip(&fun.args_types) {
                let InferenceResult { set, ty: arg_ty } = infer_expr(&expr.0, inferrer, symbols)?;

                if let Some(set) = set {
                    inferrer
                        .set_set_ty(set, TypeVar::Type(req_ty))
                        .expect("make TypeError");
                } else {
                    TypeVar::max(TypeVar::Type(req_ty), arg_ty).expect("make TypeError");
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
                let ty = symbols.variables[var]
                    .ty
                    .map_or(TypeVar::Unknown, TypeVar::Type);

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

pub struct InferenceResult {
    pub set: Option<SetId>,
    pub ty: TypeVar,
}

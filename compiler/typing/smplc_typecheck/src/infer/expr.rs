use smplc_ast::{Component, MakeSpanned, Span, Spanned};
use smplc_hir as hir;
use smplc_hir::Type;

use crate::{
    error::{TypeError, TypeResult},
    infer::Relation,
};

use super::{SetId, TypeInferrer, TypeVar};

pub fn infer_expr<'source>(
    expr: &Spanned<hir::Expr<'source>>,
    inferrer: &mut TypeInferrer,
    symbols: &hir::Symbols<'source>,
) -> TypeResult<'source, InferenceResult> {
    match &expr.0 {
        hir::Expr::Binary { lhs, op, rhs } => {
            let lhs_inference = infer_expr(&lhs, inferrer, symbols)?;
            let rhs_inference = infer_expr(&rhs, inferrer, symbols)?;

            let (ret_ty, set) = match (lhs_inference.ty, op, rhs_inference.ty) {
                (_, hir::BinOp::Add | hir::BinOp::Sub, _) => {
                    let lhs_ty = inferrer.assume_inference(lhs_inference, TypeVar::Linear)?;
                    let rhs_ty = inferrer.assume_inference(rhs_inference, TypeVar::Linear)?;

                    let operation_ty =
                        TypeVar::max(lhs_ty, rhs_ty).map_err(|(required, got)| {
                            TypeError::mismatched_types(required, got, rhs.span())
                        })?;

                    let set = inferrer
                        .try_unite(lhs_inference.set, rhs_inference.set)
                        .unwrap();

                    (operation_ty, set)
                }

                (lhs_ty, hir::BinOp::Mul | hir::BinOp::Div, rhs_ty)
                    if lhs_ty.is_number() && rhs_ty.is_number() =>
                {
                    let operation_ty =
                        TypeVar::max(lhs_ty, rhs_ty).map_err(|(required, got)| {
                            TypeError::mismatched_types(required, got, rhs.span())
                        })?;

                    let set = inferrer
                        .try_unite(lhs_inference.set, rhs_inference.set)
                        .unwrap();

                    (operation_ty, set)
                }

                (_, hir::BinOp::Mul, rhs_ty) if rhs_ty.is_vec() => {
                    inferrer.assume_inference(lhs_inference, Type::Real.into())?;

                    (rhs_ty, rhs_inference.set)
                }

                (lhs_ty, hir::BinOp::Mul | hir::BinOp::Div, _) if lhs_ty.is_vec() => {
                    inferrer.assume_inference(rhs_inference, Type::Real.into())?;

                    (lhs_ty, lhs_inference.set)
                }

                (_, hir::BinOp::Mul, _) => {
                    inferrer.assume_inference(lhs_inference, TypeVar::Linear)?;
                    inferrer.assume_inference(rhs_inference, TypeVar::Linear)?;

                    if let Some((a, b)) = lhs_inference.set.zip(rhs_inference.set) {
                        inferrer
                            .connect(Relation::Mul(a.spanned(lhs.span()), b.spanned(rhs.span())));
                    }

                    (TypeVar::Linear, None)
                }

                (_, hir::BinOp::Div, _) => {
                    inferrer.assume_inference(lhs_inference, TypeVar::Linear)?;
                    inferrer.assume_inference(rhs_inference, TypeVar::Number)?;

                    if let Some((a, b)) = lhs_inference.set.zip(rhs_inference.set) {
                        inferrer
                            .connect(Relation::Div(a.spanned(lhs.span()), b.spanned(rhs.span())));
                    }

                    (TypeVar::Linear, None)
                }

                (_, op, _) if op.is_ord() => {
                    inferrer.assume_inference(lhs_inference, TypeVar::Scalar)?;
                    inferrer.assume_inference(rhs_inference, TypeVar::Scalar)?;

                    inferrer
                        .try_unite(lhs_inference.set, rhs_inference.set)
                        .map_err(|(got, required)| {
                            TypeError::mismatched_types(required, got, rhs_inference.span)
                        })?;

                    (Type::Bool.into(), None)
                }

                (_, op, _) if op.is_eq() => {
                    inferrer.assume_inference(lhs_inference, TypeVar::Linear)?;
                    inferrer.assume_inference(rhs_inference, TypeVar::Linear)?;

                    inferrer
                        .try_unite(lhs_inference.set, rhs_inference.set)
                        .map_err(|(got, required)| {
                            TypeError::mismatched_types(required, got, rhs_inference.span)
                        })?;

                    (Type::Bool.into(), None)
                }

                (_, op, _) if op.is_logic() => {
                    inferrer.assume_inference(lhs_inference, Type::Bool.into())?;
                    inferrer.assume_inference(rhs_inference, Type::Bool.into())?;

                    let set = inferrer
                        .try_unite(lhs_inference.set, rhs_inference.set)
                        .unwrap();

                    (Type::Bool.into(), set)
                }

                _ => todo!("error"),
            };

            Ok(InferenceResult {
                set,
                ty: ret_ty,
                span: expr.span(),
            })
        }

        hir::Expr::Unary { op, rhs } => {
            let InferenceResult {
                set,
                ty: operand_ty,
                ..
            } = infer_expr(&rhs, inferrer, symbols)?;

            let min_ty = match op {
                hir::UnOp::Not => TypeVar::Type(Type::Bool),
                hir::UnOp::Neg => TypeVar::Linear,
            };

            let ret_ty = TypeVar::max(min_ty, operand_ty).map_err(|(required, got)| {
                TypeError::mismatched_types(required, got, rhs.span())
            })?;

            Ok(InferenceResult {
                set,
                ty: ret_ty,
                span: expr.span(),
            })
        }

        hir::Expr::Swizzle { lhs, swizzle } => {
            let inference = infer_expr(&lhs, inferrer, symbols)?;

            let ret_ty = match swizzle.as_slice().len() {
                1 => Type::Real,
                2 => Type::Vec2,
                3 => Type::Vec3,
                4 => Type::Vec4,

                _ => unreachable!(),
            };

            let max_component = swizzle.as_slice().into_iter().max().unwrap();

            let ty = match max_component {
                Component::X | Component::Y => TypeVar::Vec,
                Component::Z => TypeVar::Vec34,
                Component::W => Type::Vec4.into(),
            };

            inferrer.assume_inference(inference, ty)?;

            Ok(InferenceResult {
                set: None,
                ty: ret_ty.into(),
                span: expr.span(),
            })
        }

        hir::Expr::Call { fun, args } => {
            let &fun_id = fun;
            let fun = &symbols.functions[fun_id];

            for (expr, &req_ty) in args.iter().zip(&fun.args_types) {
                let InferenceResult {
                    set, ty: arg_ty, ..
                } = infer_expr(&expr, inferrer, symbols)?;

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
                span: expr.span(),
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
                span: expr.span(),
            })
        }

        hir::Expr::Atom(hir::Atom::Literal(lit)) => Ok(InferenceResult {
            set: None,
            ty: TypeVar::Type(lit.ty.into()),
            span: expr.span(),
        }),
    }
}

#[derive(Clone, Copy)]
pub struct InferenceResult {
    pub set: Option<SetId>,
    pub ty: TypeVar,
    pub span: Span,
}

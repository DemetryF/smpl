use ast::{Span, Spanned};
use smplc_ast as ast;
use smplc_hir::{ArithmOp, Atom, BinOp, Expr, NumberType, RelOp, Type, UnOp};

use crate::env::Env;
use crate::error::{SemError, SemResult};
use crate::SemCheck;

impl<'source> SemCheck<'source> for ast::Expr<'source> {
    type Checked = Expr<'source>;

    fn check(self, env: &mut Env<'source>) -> SemResult<'source, Self::Checked> {
        match self {
            ast::Expr::Infix { lhs, op, rhs } => {
                let lhs_span = lhs.span();
                let rhs_span = rhs.span();

                let lhs = Box::new(lhs.0.check(env)?);
                let rhs = Box::new(rhs.0.check(env)?);

                let op = inference_bin_op(&lhs, &rhs, op, lhs_span, rhs_span)?;

                Ok(Expr::Binary { lhs, op, rhs })
            }

            ast::Expr::Prefix { op, rhs } => {
                let span = rhs.span();
                let rhs = Box::new(rhs.0.check(env)?);

                let op = inference_un_op(op, &rhs, span)?;

                Ok(Expr::Unary { op, rhs })
            }

            ast::Expr::Call(call) => {
                let fun_ref = env.functions.get(call.id)?;

                {
                    let expected = fun_ref.args.len();
                    let received = call.args.len();

                    if expected != received {
                        return Err(SemError::invalid_arguments_count(
                            call.id.span(),
                            expected,
                            received,
                            fun_ref,
                        ));
                    }
                }

                let args = call
                    .args
                    .into_iter()
                    .zip(fun_ref.args.iter())
                    .map(|(Spanned(arg, span), &ty)| {
                        let arg = arg.check(env)?;

                        expect_ty(&arg, ty, span)?;

                        Ok(arg)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Expr::Call { fun_ref, args })
            }

            ast::Expr::Atom(atom) => Ok(Expr::Atom(match atom {
                ast::Atom::Id(id) => Atom::Var(env.variables.get(id)?),
                ast::Atom::Literal(literal) => Atom::Literal(literal),
            })),
        }
    }
}

fn inference_bin_op<'source>(
    lhs: &Expr,
    rhs: &Expr,
    op: ast::BinOp,
    lhs_span: Span,
    rhs_span: Span,
) -> SemResult<'source, BinOp> {
    let lhs_ty = expr_ty(&lhs);
    let rhs_ty = expr_ty(&rhs);

    if op.is_arithm() || op.is_rel() {
        let Ok(lhs_ty) = NumberType::try_from(lhs_ty) else {
            return Err(SemError::wrong_ty(
                lhs_span,
                lhs_ty,
                vec![Type::Real, Type::Int],
            ));
        };

        let Ok(rhs_ty) = NumberType::try_from(rhs_ty) else {
            return Err(SemError::wrong_ty(
                rhs_span,
                rhs_ty,
                vec![Type::Real, Type::Int],
            ));
        };

        if rhs_ty != lhs_ty {
            return Err(SemError::wrong_ty(
                rhs_span,
                rhs_ty.into(),
                vec![lhs_ty.into()],
            ));
        }

        let ty = lhs_ty;

        if let Ok(op) = ArithmOp::try_from(op) {
            Ok(BinOp::Arithm(op, ty))
        } else if let Ok(op) = RelOp::try_from(op) {
            Ok(BinOp::Rel(op, ty))
        } else {
            unreachable!()
        }
    } else {
        if lhs_ty != Type::Bool {
            return Err(SemError::wrong_ty(lhs_span, lhs_ty, vec![Type::Bool]));
        }

        if rhs_ty != Type::Bool {
            return Err(SemError::wrong_ty(rhs_span, rhs_ty, vec![Type::Bool]));
        }

        match op {
            ast::BinOp::Or => Ok(BinOp::Or),
            ast::BinOp::And => Ok(BinOp::And),
            _ => unreachable!(),
        }
    }
}

fn inference_un_op<'source>(op: ast::UnOp, rhs: &Expr, span: Span) -> SemResult<'source, UnOp> {
    let rhs_ty = expr_ty(rhs);

    match op {
        ast::UnOp::Not => {
            if rhs_ty == Type::Bool {
                Ok(UnOp::Not)
            } else {
                Err(SemError::wrong_ty(span, rhs_ty, vec![Type::Bool]))
            }
        }

        ast::UnOp::Neg => {
            if rhs_ty != Type::Bool {
                let ty = NumberType::try_from(rhs_ty).unwrap();
                Ok(UnOp::Neg(ty))
            } else {
                Err(SemError::wrong_ty(
                    span,
                    rhs_ty,
                    vec![Type::Real, Type::Int],
                ))
            }
        }
    }
}

pub fn expr_ty<'source>(expr: &Expr) -> Type {
    match expr {
        Expr::Binary { op, .. } => match op {
            BinOp::Rel(_, _) | BinOp::Or | BinOp::And => Type::Bool,
            &BinOp::Arithm(_, ty) => ty.into(),
        },

        Expr::Unary { op, .. } => match op {
            &UnOp::Neg(ty) => ty.into(),
            UnOp::Not => Type::Bool,
        },

        Expr::Call { fun_ref, .. } => fun_ref.ret_ty.unwrap(),

        Expr::Atom(atom) => match atom {
            Atom::Var(var_ref) => var_ref.ty,
            Atom::Literal(literal) => literal.ty,
        },
    }
}

pub fn expect_ty<'source>(expr: &Expr, ty: Type, span: Span) -> SemResult<'source, ()> {
    let expr_ty = expr_ty(&expr);

    if expr_ty != ty {
        Err(SemError::wrong_ty(span, expr_ty, vec![ty]))
    } else {
        Ok(())
    }
}

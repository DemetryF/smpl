use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

use smplc_lir::Number;
use smplc_thir as thir;
use smplc_thir::{ArithmOp, NumberType, RelOp};

use crate::idents::BaseIdents;

pub fn const_eval(expr: thir::Expr, idents: &BaseIdents) -> Number {
    match expr {
        thir::Expr::Binary { lhs, op, rhs } => {
            let lhs = const_eval(*lhs, idents);
            let rhs = const_eval(*rhs, idents);

            match op {
                thir::BinOp::Arithm(op, ty) => {
                    fn arithm<T>(op: ArithmOp, lhs: T, rhs: T) -> T
                    where
                        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
                    {
                        match op {
                            ArithmOp::Add => lhs + rhs,
                            ArithmOp::Sub => lhs - rhs,
                            ArithmOp::Mul => lhs * rhs,
                            ArithmOp::Div => lhs / rhs,
                        }
                    }

                    match ty {
                        NumberType::Real => Number::Real(arithm(op, lhs.real(), rhs.real())),
                        NumberType::Int => Number::Int(arithm(op, lhs.int(), rhs.int())),
                    }
                }
                thir::BinOp::Rel(op, ty) => {
                    fn rel(op: RelOp, ordering: Ordering) -> bool {
                        match op {
                            RelOp::Eq => ordering.is_eq(),
                            RelOp::Ne => ordering.is_ne(),
                            RelOp::Gt => ordering.is_gt(),
                            RelOp::Ge => ordering.is_ge(),
                            RelOp::Lt => ordering.is_lt(),
                            RelOp::Le => ordering.is_le(),
                        }
                    }

                    let ord = match ty {
                        NumberType::Real => f32::total_cmp(&lhs.real(), &rhs.real()),
                        NumberType::Int => Ord::cmp(&lhs.int(), &rhs.int()),
                    };

                    Number::Int(rel(op, ord) as i32)
                }
                thir::BinOp::Or => Number::Int((lhs.int() != 0 || rhs.int() != 0) as _),
                thir::BinOp::And => Number::Int((lhs.int() != 0 && rhs.int() != 0) as _),
            }
        }
        thir::Expr::Unary { op, rhs } => {
            let rhs = const_eval(*rhs, idents);

            match op {
                thir::UnOp::Neg(NumberType::Real) => Number::Real(-rhs.real()),
                thir::UnOp::Neg(NumberType::Int) => Number::Int(-rhs.int()),
                thir::UnOp::Not => Number::Int(!rhs.int()),
            }
        }
        thir::Expr::Call { .. } => panic!("there's no const fn lol"),
        thir::Expr::Atom(atom) => match atom {
            thir::Atom::Var(var) => idents.constants[&idents.get(var)],
            thir::Atom::Literal(literal) => match literal.ty.into() {
                thir::Type::Real => Number::Real(parse_int::parse(literal.value).unwrap()),
                thir::Type::Int => Number::Int(parse_int::parse(literal.value).unwrap()),
                thir::Type::Bool => Number::Int(if literal.value == "true" {
                    1
                } else if literal.value == "false" {
                    0
                } else {
                    unreachable!()
                }),
            },
        },
    }
}

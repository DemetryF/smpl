use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

use num::Complex;

use smplc_lir::Value;
use smplc_thir::{self as thir, VecOp};
use smplc_thir::{ArithmOp, NumberType, RelOp};

use crate::idents::BaseIdents;

pub fn const_eval(expr: thir::Expr, idents: &BaseIdents) -> Value {
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
                        NumberType::Complex => {
                            Value::Complex(arithm(op, lhs.complex(), rhs.complex()))
                        }
                        NumberType::Real => Value::Real(arithm(op, lhs.real(), rhs.real())),
                        NumberType::Int => Value::Int(arithm(op, lhs.int(), rhs.int())),
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

                        NumberType::Complex => unreachable!(),
                    };

                    Value::Int(rel(op, ord) as i32)
                }
                thir::BinOp::Vec(op, ty) => {
                    ();

                    if matches!(op, thir::VecOp::Add | thir::VecOp::Sub) {
                        fn oper<T>(lhs: T, op: VecOp, rhs: T) -> T
                        where
                            T: Add<Output = T> + Sub<Output = T>,
                        {
                            match op {
                                VecOp::Add => lhs + rhs,
                                VecOp::Sub => lhs - rhs,
                                _ => unreachable!(),
                            }
                        }

                        return match ty {
                            thir::VecType::Vec2 => Value::Vec2(oper(lhs.vec2(), op, rhs.vec2())),
                            thir::VecType::Vec3 => Value::Vec3(oper(lhs.vec3(), op, rhs.vec3())),
                            thir::VecType::Vec4 => Value::Vec4(oper(lhs.vec4(), op, rhs.vec4())),
                        };
                    }

                    let (scalar, vec) = match op {
                        thir::VecOp::LeftMul => (lhs.real(), rhs),
                        thir::VecOp::RightMul => (rhs.real(), lhs),
                        thir::VecOp::Div => (lhs.real(), rhs),

                        _ => unreachable!(),
                    };

                    fn oper<V>(scalar: f32, op: VecOp, vec: V) -> V
                    where
                        V: Mul<f32, Output = V>,
                        V: Div<f32, Output = V>,
                    {
                        match op {
                            VecOp::LeftMul | VecOp::RightMul => vec * scalar,
                            VecOp::Div => vec / scalar,
                            _ => unreachable!(),
                        }
                    }

                    match ty {
                        thir::VecType::Vec2 => Value::Vec2(oper(scalar, op, vec.vec2())),
                        thir::VecType::Vec3 => Value::Vec3(oper(scalar, op, vec.vec3())),
                        thir::VecType::Vec4 => Value::Vec4(oper(scalar, op, vec.vec4())),
                    }
                }
                thir::BinOp::Or => Value::Int((lhs.int() != 0 || rhs.int() != 0) as _),
                thir::BinOp::And => Value::Int((lhs.int() != 0 && rhs.int() != 0) as _),
            }
        }
        thir::Expr::Unary { op, rhs } => {
            let rhs = const_eval(*rhs, idents);

            match op {
                thir::UnOp::Neg(thir::LinearType::Vec(thir::VecType::Vec2)) => {
                    Value::Vec2(-rhs.vec2())
                }
                thir::UnOp::Neg(thir::LinearType::Vec(thir::VecType::Vec3)) => {
                    Value::Vec3(-rhs.vec3())
                }
                thir::UnOp::Neg(thir::LinearType::Vec(thir::VecType::Vec4)) => {
                    Value::Vec4(-rhs.vec4())
                }
                thir::UnOp::Neg(thir::LinearType::Number(NumberType::Complex)) => {
                    Value::Complex(-rhs.complex())
                }
                thir::UnOp::Neg(thir::LinearType::Number(NumberType::Real)) => {
                    Value::Real(-rhs.real())
                }
                thir::UnOp::Neg(thir::LinearType::Number(NumberType::Int)) => {
                    Value::Int(-rhs.int())
                }
                thir::UnOp::Not => Value::Int(!rhs.int()),
            }
        }
        thir::Expr::Call { .. } => panic!("there's no const fn lol"),
        thir::Expr::Atom(atom) => match atom {
            thir::Atom::Var(var) => idents.constants[&idents.get(var)],
            thir::Atom::Literal(literal) => match literal.ty {
                thir::LiteralType::Complex => {
                    Value::Complex(Complex::new(0.0, parse_int::parse(literal.value).unwrap()))
                }
                thir::LiteralType::Real => Value::Real(parse_int::parse(literal.value).unwrap()),
                thir::LiteralType::Int => Value::Int(parse_int::parse(literal.value).unwrap()),
                thir::LiteralType::Bool => Value::Int(if literal.value == "true" { 1 } else { 0 }),
            },
        },
    }
}

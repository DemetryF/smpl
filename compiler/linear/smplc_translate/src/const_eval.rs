use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

use nalgebra::{Vector2, Vector3, Vector4};
use num::Complex;

use smplc_lir::Value;
use smplc_thir::{self as thir, VecOp};
use smplc_thir::{ArithmOp, NumberType, OrdOp};

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
                            let lhs = lhs.f32x2();
                            let rhs = rhs.f32x2();

                            let lhs = Complex::new(lhs.x, lhs.y);
                            let rhs = Complex::new(rhs.x, rhs.y);

                            let res = arithm(op, lhs, rhs);

                            let res = Vector2::new(res.re, res.im);

                            Value::F32x2(res)
                        }
                        NumberType::Real => Value::Real(arithm(op, lhs.real(), rhs.real())),
                        NumberType::Int => Value::Int(arithm(op, lhs.int(), rhs.int())),
                    }
                }
                thir::BinOp::Ord(op, ty) => {
                    fn rel(op: OrdOp, ordering: Ordering) -> bool {
                        match op {
                            OrdOp::Gt => ordering.is_gt(),
                            OrdOp::Ge => ordering.is_ge(),
                            OrdOp::Lt => ordering.is_lt(),
                            OrdOp::Le => ordering.is_le(),
                        }
                    }

                    let ord = match ty {
                        NumberType::Real => f32::total_cmp(&lhs.real(), &rhs.real()),
                        NumberType::Int => Ord::cmp(&lhs.int(), &rhs.int()),

                        NumberType::Complex => unreachable!(),
                    };

                    Value::Int(rel(op, ord) as i32)
                }
                thir::BinOp::Eq(op, ty) => {
                    let value = match ty {
                        thir::LinearType::Number(NumberType::Complex)
                        | thir::LinearType::Vec(thir::VecType::Vec2) => lhs.f32x2() == rhs.f32x2(),
                        thir::LinearType::Vec(thir::VecType::Vec3) => lhs.f32x3() == rhs.f32x3(),
                        thir::LinearType::Vec(thir::VecType::Vec4) => lhs.f32x4() == rhs.f32x4(),

                        thir::LinearType::Number(NumberType::Real) => lhs.real() == rhs.real(),
                        thir::LinearType::Number(NumberType::Int) => lhs.int() == rhs.int(),
                    };

                    Value::Int(!(value ^ (op == thir::EqOp::Eq)) as i32)
                }
                thir::BinOp::Vec(op, ty) => {
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
                            thir::VecType::Vec2 => Value::F32x2(oper(lhs.f32x2(), op, rhs.f32x2())),
                            thir::VecType::Vec3 => Value::F32x3(oper(lhs.f32x3(), op, rhs.f32x3())),
                            thir::VecType::Vec4 => Value::F32x4(oper(lhs.f32x4(), op, rhs.f32x4())),
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
                        thir::VecType::Vec2 => Value::F32x2(oper(scalar, op, vec.f32x2())),
                        thir::VecType::Vec3 => Value::F32x3(oper(scalar, op, vec.f32x3())),
                        thir::VecType::Vec4 => Value::F32x4(oper(scalar, op, vec.f32x4())),
                    }
                }
                thir::BinOp::Or => Value::Int((lhs.int() != 0 || rhs.int() != 0) as _),
                thir::BinOp::And => Value::Int((lhs.int() != 0 && rhs.int() != 0) as _),
            }
        }
        thir::Expr::Unary { op, rhs } => {
            let rhs = const_eval(*rhs, idents);

            match op {
                thir::UnOp::Neg(thir::LinearType::Number(NumberType::Complex))
                | thir::UnOp::Neg(thir::LinearType::Vec(thir::VecType::Vec2)) => {
                    Value::F32x2(-rhs.f32x2())
                }
                thir::UnOp::Neg(thir::LinearType::Vec(thir::VecType::Vec3)) => {
                    Value::F32x3(-rhs.f32x3())
                }
                thir::UnOp::Neg(thir::LinearType::Vec(thir::VecType::Vec4)) => {
                    Value::F32x4(-rhs.f32x4())
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
        thir::Expr::Swizzle { lhs, swizzle } => {
            let lhs = const_eval(*lhs, idents);

            let source_vec = match &lhs {
                Value::F32x2(matrix) => matrix.as_slice(),
                Value::F32x3(matrix) => matrix.as_slice(),
                Value::F32x4(matrix) => matrix.as_slice(),

                _ => unreachable!(),
            };

            let mut new_vec = [0.; 4];

            for (n, &comp) in swizzle.as_slice().into_iter().enumerate() {
                new_vec[n] = source_vec[comp as usize];
            }

            match swizzle.as_slice().len() {
                1 => Value::Real(new_vec[0]),
                2 => Value::F32x2(Vector2::new(new_vec[0], new_vec[1])),
                3 => Value::F32x3(Vector3::new(new_vec[0], new_vec[1], new_vec[2])),
                4 => Value::F32x4(Vector4::new(new_vec[0], new_vec[1], new_vec[2], new_vec[3])),

                _ => unreachable!(),
            }
        }
        thir::Expr::Call { .. } => panic!("there's no const fn lol"),
        thir::Expr::Atom(atom) => match atom {
            thir::Atom::Var(var) => idents.constants[&idents.get(var)],
            thir::Atom::Literal(literal) => match literal.ty {
                thir::LiteralType::Complex => {
                    Value::F32x2(Vector2::new(0.0, parse_int::parse(literal.value).unwrap()))
                }
                thir::LiteralType::Real => Value::Real(parse_int::parse(literal.value).unwrap()),
                thir::LiteralType::Int => Value::Int(parse_int::parse(literal.value).unwrap()),
                thir::LiteralType::Bool => Value::Int(if literal.value == "true" { 1 } else { 0 }),
            },
        },
    }
}

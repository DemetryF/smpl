use num::complex::Complex32;

use smplc_lir::{Atom, ControlFlow, Id, Sequental, Type, UnOp, Value};
use smplc_thir as thir;
use smplc_thir::Symbols;

use crate::{
    call::translate_call, idents::BaseIdents, logic::translate_logic, translator::Translator,
};

pub fn translate_expr<'source>(
    expr: thir::Expr,
    translator: &mut Translator<'source>,
    idents: &mut BaseIdents,
    symbols: &Symbols<'source>,
) -> Id {
    match expr {
        thir::Expr::Binary {
            lhs,
            op: thir::BinOp::Arithm(op, ty),
            rhs,
        } => {
            let ty = ty.into();

            let lhs = translate_expr(*lhs, translator, idents, symbols);
            let rhs = translate_expr(*rhs, translator, idents, symbols);

            let result = idents.next(ty);

            translator.code.push(Sequental::Binary {
                dst: result,
                op,
                ty,
                lhs: Atom::Id(lhs),
                rhs: Atom::Id(rhs),
            });

            result
        }

        thir::Expr::Binary {
            lhs,
            op: thir::BinOp::Vec(op, ty),
            rhs,
        } => {
            let ty = ty.into();

            let mut lhs = translate_expr(*lhs, translator, idents, symbols);
            let mut rhs = translate_expr(*rhs, translator, idents, symbols);

            let result = idents.next(ty);

            let op = match op {
                thir::VecOp::Add => thir::ArithmOp::Add,
                thir::VecOp::Sub => thir::ArithmOp::Sub,
                thir::VecOp::RightMul => thir::ArithmOp::Mul,
                thir::VecOp::Div => thir::ArithmOp::Div,
                thir::VecOp::LeftMul => {
                    std::mem::swap(&mut lhs, &mut rhs);

                    thir::ArithmOp::Mul
                }
            };

            translator.code.push(Sequental::Binary {
                dst: result,
                op,
                ty,
                lhs: Atom::Id(lhs),
                rhs: Atom::Id(rhs),
            });

            result
        }

        thir::Expr::Unary {
            op: thir::UnOp::Neg(ty),
            rhs,
        } => {
            let ty = ty.into();

            let rhs = translate_expr(*rhs, translator, idents, symbols);

            let result = idents.next(ty);

            translator.code.push(Sequental::Unary {
                dst: result,
                op: UnOp::Neg,
                ty,
                operand: Atom::Id(rhs),
            });

            result
        }

        thir::Expr::Swizzle { lhs, swizzle } => {
            let lhs = translate_expr(*lhs, translator, idents, symbols);

            let ty = match swizzle.combination.len() {
                1 => Type::Real,
                2 => Type::Vec2,
                3 => Type::Vec3,
                4 => Type::Vec4,
                _ => unreachable!(),
            };

            let result = idents.next(ty);

            translator.code.push(Sequental::Unary {
                dst: result,
                op: UnOp::Swizzle(swizzle),
                ty,
                operand: Atom::Id(lhs),
            });

            result
        }

        thir::Expr::Call { fun, args } => {
            let ret_ty = symbols.functions[fun].ret_ty.unwrap();

            let result = idents.next(ret_ty.into());

            translate_call(translator, idents, symbols, Some(result), fun, args);

            result
        }

        thir::Expr::Atom(atom) => {
            let value = translate_atom(atom, idents);

            let result = idents.next(value.ty());

            translator
                .code
                .push(Sequental::Assign { dst: result, value });

            result
        }

        thir::Expr::Binary { .. } | thir::Expr::Unary { .. } => {
            let true_label = translator.next_label();
            let false_label = translator.next_label();
            let end_label = translator.next_label();

            translate_logic(expr, translator, idents, symbols, true_label, false_label);

            let result = idents.next(Type::Int);

            translator.code.label(true_label);
            translator.code.push(Sequental::Assign {
                dst: result,
                value: Atom::Value(Value::Int(1)),
            });
            translator.code.push(ControlFlow::Goto { label: end_label });

            translator.code.label(false_label);
            translator.code.push(Sequental::Assign {
                dst: result,
                value: Atom::Value(Value::Int(0)),
            });

            translator.code.label(end_label);

            result
        }
    }
}

pub fn translate_atom(atom: thir::Atom, idents: &mut BaseIdents) -> Atom {
    match atom {
        thir::Atom::Var(var) => Atom::Id(idents.get(var)),
        thir::Atom::Literal(literal) => Atom::Value(match literal.ty {
            thir::LiteralType::Complex => Value::Complex(Complex32::new(
                0.0,
                parse_int::parse(&literal.value[0..literal.value.len() - 1]).unwrap(),
            )),
            thir::LiteralType::Real => Value::Real(parse_int::parse(literal.value).unwrap()),
            thir::LiteralType::Int => Value::Int(parse_int::parse(literal.value).unwrap()),
            thir::LiteralType::Bool => Value::Int(if literal.value == "true" { 1 } else { 0 }),
        }),
    }
}

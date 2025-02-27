mod expr;
mod statement;
mod translator;

use std::collections::HashMap;

use smplc_lir::{Code, CodeFunction, FunctionId, Id, Number};
use smplc_thir::{self as thir, Symbols};
use smplc_thir::{ArithmOp, BinOp, Block, Expr, NumberType, RelOp, THIR};

use translator::Translator;

trait Translate: Sized {
    fn translate(self, translator: &mut Translator, symbols: &Symbols);
}

impl Translate for Block<'_> {
    fn translate(self, translator: &mut Translator, symbols: &Symbols) {
        self.statements
            .into_iter()
            .for_each(|stmt| stmt.translate(translator, symbols))
    }
}

pub fn translate(thir: THIR) -> (Code, HashMap<Id, NumberType>) {
    let THIR {
        constants,
        functions,
        symbols,
    } = thir;

    let mut translator = Translator::default();

    for constant in constants {
        let value = eval_constant_expr(constant.value, &translator);

        let id = translator
            .variables
            .add(constant.id, symbols.variables[constant.id].ty);

        translator.code.constants.insert(id, value);
    }

    for function in functions {
        let args = function
            .args
            .into_iter()
            .map(|var| translator.variables.add(var, symbols.variables[var].ty))
            .collect();

        translator.code.add_function(CodeFunction::new(
            FunctionId(symbols.functions[function.id].id.0.into()),
            args,
        ));

        for stmt in function.body.statements {
            stmt.translate(&mut translator, &symbols)
        }
    }

    (translator.code, translator.variables.types)
}

fn eval_constant_expr(expr: Expr, translator: &Translator) -> Number {
    match expr {
        Expr::Binary { lhs, op, rhs } => {
            let lhs = eval_constant_expr(*lhs, translator);
            let rhs = eval_constant_expr(*rhs, translator);

            match op {
                BinOp::Or => {
                    let lhs = lhs.int() == 1;
                    let rhs = rhs.int() == 1;

                    Number::Int((lhs || rhs) as i32)
                }

                BinOp::And => {
                    let lhs = lhs.int() == 1;
                    let rhs = rhs.int() == 1;

                    Number::Int((lhs && rhs) as i32)
                }

                BinOp::Arithm(op, ty) => match ty {
                    NumberType::Real => {
                        let lhs = lhs.real();
                        let rhs = rhs.real();

                        let value = match op {
                            ArithmOp::Add => lhs + rhs,
                            ArithmOp::Sub => lhs - rhs,
                            ArithmOp::Mul => lhs * rhs,
                            ArithmOp::Div => lhs / rhs,
                        };

                        Number::Real(value)
                    }

                    NumberType::Int => {
                        let lhs = lhs.int();
                        let rhs = rhs.int();

                        let value = match op {
                            ArithmOp::Add => lhs + rhs,
                            ArithmOp::Sub => lhs - rhs,
                            ArithmOp::Mul => lhs * rhs,
                            ArithmOp::Div => lhs / rhs,
                        };

                        Number::Int(value)
                    }
                },

                BinOp::Rel(op, ty) => match ty {
                    NumberType::Real => {
                        let lhs = lhs.real();
                        let rhs = rhs.real();

                        let value = match op {
                            RelOp::Eq => lhs == rhs,
                            RelOp::Ne => lhs != rhs,
                            RelOp::Gt => lhs > rhs,
                            RelOp::Ge => lhs >= rhs,
                            RelOp::Lt => lhs < rhs,
                            RelOp::Le => lhs <= rhs,
                        };

                        Number::Int(value as i32)
                    }

                    NumberType::Int => {
                        let lhs = lhs.int();
                        let rhs = rhs.int();

                        let value = match op {
                            RelOp::Eq => lhs == rhs,
                            RelOp::Ne => lhs != rhs,
                            RelOp::Gt => lhs > rhs,
                            RelOp::Ge => lhs >= rhs,
                            RelOp::Lt => lhs < rhs,
                            RelOp::Le => lhs <= rhs,
                        };

                        Number::Int(value as i32)
                    }
                },
            }
        }

        Expr::Unary { op, rhs } => {
            let rhs = eval_constant_expr(*rhs, translator);

            match op {
                thir::UnOp::Not => Number::Int((rhs != Number::Int(0)) as i32),
                thir::UnOp::Neg(ty) => match ty {
                    NumberType::Real => Number::Real(-rhs.real()),
                    NumberType::Int => Number::Int(-rhs.int()),
                },
            }
        }

        Expr::Call { .. } => panic!("function call in constant expression"),

        Expr::Atom(atom) => match atom {
            thir::Atom::Var(var_ref) => {
                let id = translator.variables.get(var_ref);

                *translator.code.constants.get(&id).unwrap()
            }

            thir::Atom::Literal(literal) => Number::from(literal),
        },
    }
}

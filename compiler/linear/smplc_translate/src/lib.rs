mod expr;
mod statement;
mod translator;

use smplc_hir::{Block, Expr, HIR};
use smplc_ir::{BinOp, Code, CodeFunction, FunctionId, Halt, UnOp};

use translator::Translator;

trait Translate: Sized {
    fn translate(self, translator: &mut Translator);
}

impl Translate for Block {
    fn translate(self, translator: &mut Translator) {
        self.statements
            .into_iter()
            .for_each(|stmt| stmt.translate(translator))
    }
}

pub fn translate(hir: HIR) -> Code {
    let HIR {
        constants,
        functions,
    } = hir;

    let mut translator = Translator::default();

    for constant in constants {
        let value = eval_constant_expr(constant.value, &translator);

        let id = translator.variables.add(constant.data);

        translator.code.constants.insert(id, value);
    }

    for function in functions {
        let args = function
            .args
            .into_iter()
            .map(|var_ref| translator.variables.add(var_ref))
            .collect();

        translator.code.add_function(CodeFunction {
            id: FunctionId(function.data.id.clone()),
            args,
            ..Default::default()
        });

        function
            .body
            .into_iter()
            .for_each(|stmt| stmt.translate(&mut translator))
    }

    translator.code.push(Halt);

    translator.code
}

fn eval_constant_expr(expr: Expr, translator: &Translator) -> f32 {
    match expr {
        Expr::Binary { lhs, op, rhs } => {
            let lhs = eval_constant_expr(*lhs, translator);
            let rhs = eval_constant_expr(*rhs, translator);

            match op {
                BinOp::Or => ((lhs == 1.0) || (rhs == 1.0)) as i32 as f32,
                BinOp::And => ((lhs == 1.0) && (rhs == 1.0)) as i32 as f32,

                BinOp::Ne => (lhs != rhs) as i32 as f32,
                BinOp::Eq => (lhs == rhs) as i32 as f32,
                BinOp::Ge => (lhs >= rhs) as i32 as f32,
                BinOp::Gt => (lhs > rhs) as i32 as f32,
                BinOp::Le => (lhs <= rhs) as i32 as f32,
                BinOp::Lt => (lhs < rhs) as i32 as f32,

                BinOp::Add => lhs + rhs,
                BinOp::Sub => lhs - rhs,
                BinOp::Mul => lhs * rhs,
                BinOp::Div => lhs / rhs,
            }
        }

        Expr::Unary { op, rhs } => {
            let rhs = eval_constant_expr(*rhs, translator);

            match op {
                UnOp::Not => !(rhs == 1.0) as i32 as f32,
                UnOp::Neg => -rhs,
            }
        }

        Expr::Call { .. } => panic!("function call in constant expression"),

        Expr::Atom(atom) => match atom {
            smplc_hir::Atom::Var(var_ref) => {
                let id = translator.variables.get(var_ref);

                *translator.code.constants.get(&id).unwrap()
            }

            smplc_hir::Atom::Literal(literal) => f32::from(literal),
        },
    }
}

use std::fmt::Write;

use smplc_ir::{Atom, BinOp, Binary};

use crate::builder::Builder;
use crate::env::Env;

use crate::compile::Compile;

impl Compile for Binary {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> std::fmt::Result {
        let result_ptr = env.get_or_add(self.result);

        let instruction = match self.op {
            BinOp::Add => "addss",
            BinOp::Sub => "subss",
            BinOp::Mul => "mulss",
            BinOp::Div => "divss",

            BinOp::And => "and",
            BinOp::Or => "or",

            BinOp::Eq => "sete",
            BinOp::Ne => "setne",
            BinOp::Ge => "setae",
            BinOp::Gt => "seta",
            BinOp::Le => "setbe",
            BinOp::Lt => "setb",
        };

        match self.op {
            BinOp::Add | BinOp::Div | BinOp::Mul | BinOp::Sub => {
                let (lhs, rhs) = match (self.lhs, self.rhs) {
                    (Atom::Id(lhs), Atom::Id(rhs)) => {
                        let lhs = env.get(lhs);
                        let rhs = env.get(rhs);

                        (lhs, rhs)
                    }

                    (Atom::Id(lhs), Atom::Number(rhs)) => {
                        let lhs = env.get(lhs);
                        let rhs = builder.float(rhs);

                        (lhs, rhs)
                    }

                    (Atom::Number(lhs), Atom::Id(rhs)) => {
                        let lhs = builder.float(lhs);
                        let rhs = env.get(rhs);

                        (lhs, rhs)
                    }

                    (Atom::Number(lhs), Atom::Number(rhs)) => {
                        let result = match self.op {
                            BinOp::Add => lhs + rhs,
                            BinOp::Sub => lhs - rhs,
                            BinOp::Mul => lhs * rhs,
                            BinOp::Div => lhs / rhs,

                            _ => unreachable!(),
                        };

                        let result = builder.float(result);

                        writeln!(builder, "movss xmm0, {result}")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")?;

                        return Ok(());
                    }
                };

                writeln!(builder, "movss xmm0, {lhs}")?;
                writeln!(builder, "{instruction} xmm0, {rhs}")?;
                writeln!(builder, "movss {result_ptr}, xmm0")
            }

            BinOp::And | BinOp::Or => {
                match (self.lhs, self.rhs) {
                    (Atom::Id(lhs), Atom::Id(rhs)) => {
                        let lhs = env.get(lhs);
                        let rhs = env.get(rhs);

                        let one = builder.float(1.0);

                        // xmm0 = 1
                        writeln!(builder, "movss xmm0, {one}")?;
                        // xmm1 = lhs
                        writeln!(builder, "movss xmm1, {lhs}")?;

                        // eax = 1 == lhs
                        writeln!(builder, "ucomiss xmm0, xmm1")?;
                        writeln!(builder, "sete al")?;
                        writeln!(builder, "movzx eax, al")?;

                        // xmm1 = rhs
                        writeln!(builder, "movss xmm1, {rhs}")?;
                        // ebx = 1 == rhs
                        writeln!(builder, "ucomiss xmm0, xmm1")?;
                        writeln!(builder, "sete bl")?;
                        writeln!(builder, "movzx ebx, bl")?;

                        writeln!(builder, "{instruction} ebx, eax")?;
                        writeln!(builder, "cvtsi2ss xmm0, ebx")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")
                    }
                    (Atom::Id(id), Atom::Number(num)) | (Atom::Number(num), Atom::Id(id)) => {
                        let bool = num == 1.0;

                        match (self.op, bool) {
                            // b | true == true
                            // b & false == false
                            (BinOp::Or, true) | (BinOp::And, false) => {
                                let num = bool as i32 as f32;
                                let result = builder.float(num);

                                writeln!(builder, "movss xmm0, {result}")?;
                                writeln!(builder, "movss {result_ptr}, xmm0")
                            }

                            // b | false = b
                            // b & true = b
                            (BinOp::Or, false) | (BinOp::And, true) => {
                                let id = env.get(id);

                                writeln!(builder, "movss xmm0, {id}")?;
                                writeln!(builder, "movss {result_ptr}, xmm0")
                            }

                            _ => unreachable!(),
                        }
                    }

                    (Atom::Number(lhs), Atom::Number(rhs)) => {
                        let lhs = lhs == 1.0;
                        let rhs = rhs == 1.0;

                        let result = match self.op {
                            BinOp::Or => lhs || rhs,
                            BinOp::And => lhs && rhs,

                            _ => unreachable!(),
                        };

                        let result = builder.float(result as i32 as f32);

                        writeln!(builder, "movss xmm0, {result}")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")
                    }
                }
            }

            BinOp::Ne | BinOp::Eq | BinOp::Ge | BinOp::Gt | BinOp::Le | BinOp::Lt => {
                let lhs = match self.lhs {
                    Atom::Id(id) => env.get(id),
                    Atom::Number(num) => builder.float(num),
                };

                let rhs = match self.rhs {
                    Atom::Id(id) => env.get(id),
                    Atom::Number(num) => builder.float(num),
                };

                writeln!(builder, "movss xmm0, {lhs}")?;
                writeln!(builder, "movss xmm1, {rhs}")?;

                writeln!(builder, "ucomiss xmm0, xmm1")?;
                writeln!(builder, "{instruction} al")?;
                writeln!(builder, "movzx eax, al")?;
                writeln!(builder, "cvtsi2ss xmm0, eax")?;
                writeln!(builder, "movss {result_ptr}, xmm0")
            }
        }
    }
}

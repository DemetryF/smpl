use std::fmt::{self, Write};

use smplc_lir::{ArithmOp, BinOp, Dims, F32sOp, Id, Sequental, Type, UnOp};

use crate::{
    builder::Builder,
    compile::atom,
    env::{Address, Env},
    STACK_ALIGN,
};

use super::Compile;

impl Compile for Sequental<'_> {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        let dst = match self {
            Sequental::Assign { dst, .. }
            | Sequental::Binary { dst, .. }
            | Sequental::Unary { dst, .. } => Some(dst),
            Sequental::Call { dst, .. } => dst,
        };

        match self {
            Sequental::Assign { dst, value } => {
                let result_ptr = env.get_or_add(dst);

                let value = atom(env, builder, value);

                match dst.ty() {
                    Type::Int => {
                        writeln!(builder, "mov eax, {value}")?;
                        writeln!(builder, "mov {result_ptr}, eax")?;
                    }
                    Type::Real | Type::F32x2 | Type::F32x3 | Type::F32x4 => {
                        writeln!(builder, "movups xmm0, {value}")?;
                        writeln!(builder, "movaps {result_ptr}, xmm0")?;
                    }
                }
            }

            Sequental::Binary { dst, op, lhs, rhs } => {
                let res = env.get_or_add(dst);

                let lhs = atom(env, builder, lhs);
                let rhs = atom(env, builder, rhs);

                match op {
                    BinOp::Int(op @ (ArithmOp::Add | ArithmOp::Sub | ArithmOp::Mul)) => {
                        let instr = match op {
                            ArithmOp::Add => "add",
                            ArithmOp::Sub => "sub",
                            ArithmOp::Mul => "imul",
                            _ => unreachable!(),
                        };

                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "{instr} eax, {rhs}")?;
                        writeln!(builder, "mov {res}, eax")?;
                    }

                    BinOp::Int(ArithmOp::Div) => {
                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "mov ebx, {rhs}")?;
                        writeln!(builder, "idiv ebx")?;
                        writeln!(builder, "mov {res}, eax")?;
                    }

                    BinOp::Int(
                        op @ (ArithmOp::Eq
                        | ArithmOp::Ne
                        | ArithmOp::Lt
                        | ArithmOp::Le
                        | ArithmOp::Gt
                        | ArithmOp::Ge),
                    ) => {
                        let cc = match op {
                            ArithmOp::Eq => "e",
                            ArithmOp::Ne => "ne",
                            ArithmOp::Lt => "l",
                            ArithmOp::Le => "le",
                            ArithmOp::Gt => "g",
                            ArithmOp::Ge => "ge",
                            _ => unreachable!(),
                        };

                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "cmp eax, {rhs}")?;
                        writeln!(builder, "set{cc} {res}")?;
                    }

                    BinOp::Real(
                        op @ (ArithmOp::Add | ArithmOp::Sub | ArithmOp::Mul | ArithmOp::Div),
                    ) => {
                        let instr = match op {
                            ArithmOp::Add => "addss",
                            ArithmOp::Sub => "subss",
                            ArithmOp::Mul => "mulss",
                            ArithmOp::Div => "divss",
                            _ => unreachable!(),
                        };

                        writeln!(builder, "movss xmm0, {lhs}")?;
                        writeln!(builder, "{instr} xmm0, {rhs}")?;
                        writeln!(builder, "movss {res}, xmm0")?;
                    }

                    BinOp::Real(
                        op @ (ArithmOp::Eq
                        | ArithmOp::Ne
                        | ArithmOp::Lt
                        | ArithmOp::Le
                        | ArithmOp::Gt
                        | ArithmOp::Ge),
                    ) => {
                        let cc = match op {
                            ArithmOp::Eq => "e",
                            ArithmOp::Ne => "ne",
                            ArithmOp::Lt => "b",
                            ArithmOp::Le => "be",
                            ArithmOp::Gt => "a",
                            ArithmOp::Ge => "ae",
                            _ => unreachable!(),
                        };

                        writeln!(builder, "movss xmm0, {lhs}")?;
                        writeln!(builder, "ucomiss xmm0, {rhs}")?;
                        writeln!(builder, "set{cc} {res}")?;
                    }

                    BinOp::F32s(_, op @ (F32sOp::Add | F32sOp::Sub)) => {
                        let instr = match op {
                            F32sOp::Add => "addps",
                            F32sOp::Sub => "subps",
                            _ => unreachable!(),
                        };

                        writeln!(builder, "movups xmm0, {lhs}")?;
                        writeln!(builder, "{instr} xmm0, {rhs}")?;
                        writeln!(builder, "movaps {res}, xmm0")?;
                    }

                    BinOp::F32s(_, op @ (F32sOp::ScalarMul | F32sOp::ScalarDiv)) => {
                        let instr = match op {
                            F32sOp::ScalarMul => "mulps",
                            F32sOp::ScalarDiv => "divps",
                            _ => unreachable!(),
                        };

                        writeln!(builder, "movups xmm0, {lhs}")?;
                        writeln!(builder, "movss xmm1, {rhs}")?;
                        writeln!(builder, "shufps xmm1, xmm1, 0b00_00_00_00")?;
                        writeln!(builder, "{instr} xmm0, xmm1")?;
                        writeln!(builder, "movaps {res}, xmm0")?;
                    }

                    BinOp::F32s(dims, op @ (F32sOp::Eq | F32sOp::Ne)) => {
                        let cc = match op {
                            F32sOp::Eq => "e",
                            F32sOp::Ne => "ne",
                            _ => unreachable!(),
                        };

                        let mask = match dims {
                            Dims::X2 => "0b11",
                            Dims::X3 => "0b111",
                            Dims::X4 => "0b1111",
                        };

                        writeln!(builder, "movaps  xmm0, {lhs}")?;
                        writeln!(builder, "movaps  xmm1, {rhs}")?;
                        writeln!(builder, "cmpeqps xmm0, xmm1")?;
                        writeln!(builder, "movmskps eax, xmm0")?;
                        writeln!(builder, "and eax, {mask}")?;
                        writeln!(builder, "cmp eax, {mask}")?;
                        writeln!(builder, "set{cc} {res}")?;
                    }

                    BinOp::ComplexMul => {
                        writeln!(builder, "movups xmm0, {lhs}")?;
                        writeln!(builder, "movups xmm1, {rhs}")?;
                        writeln!(builder, "shufps xmm0, xmm0, 0b00_01_01_00")?; // xmm0 = [ a, b, b, a ]
                        writeln!(builder, "shufps xmm1, xmm1, 0b01_00_01_00")?; // xmm1 = [ c, d, c, d ]
                        writeln!(builder, "mulps  xmm1, xmm0")?; // xmm1 = [ac, bd, bc, ad]
                        writeln!(builder, "movaps xmm0, xmm1")?; // xmm0 = [ac, bd, bc, ad]
                        writeln!(builder, "hsubps xmm1, xmm1")?; // xmm1 = [ac - bd, bc - ad]
                        writeln!(builder, "haddps xmm0, xmm0")?; // xmm0 = [ac + bd, bc + ad]
                        writeln!(builder, "movss xmm0, xmm1")?; //  xmm0 = [ac - bd, bc + ad]
                        writeln!(builder, "movaps {res}, xmm0")?;
                    }

                    BinOp::ComplexDiv => {
                        writeln!(builder, "movups xmm0, {lhs}")?;
                        writeln!(builder, "movups xmm1, {rhs}")?;
                        writeln!(builder, "movaps xmm2, xmm1")?; // xmm2 = [ c,  d  ]
                        writeln!(builder, "mulps  xmm2, xmm2")?; // xmm2 = [ cc, dd ]
                        writeln!(builder, "haddps xmm2, xmm2")?; // xmm2 = [ cc + dd ]
                        writeln!(builder, "shufps xmm0, xmm0, 0b00_01_01_00")?; // xmm0 = [ a, b, b, a ]
                        writeln!(builder, "shufps xmm1, xmm1, 0b01_00_01_00")?; // xmm1 = [ c, d, c, d ]
                        writeln!(builder, "mulps  xmm1, xmm1")?; // xmm1 = [ac, bd, bc, ad ]
                        writeln!(builder, "movaps xmm0, xmm1")?; // xmm0 = [ac, bd, bc, ad ]
                        writeln!(builder, "haddps xmm1, xmm1")?; // xmm1 = [ac + bd, bc + ad]
                        writeln!(builder, "hsubps xmm0, xmm0")?; // xmm0 = [ac - bd, bc - ad]
                        writeln!(builder, "movss  xmm0, xmm1")?; // xmm0 = [ac + bd, bc - ad]
                        writeln!(builder, "divps  xmm0, xmm2")?; // xmm0 = [(ac + bd)/(cc + dd), (bc - ad)/(cc + dd)]
                        writeln!(builder, "movaps {res}, xmm0")?;
                    }
                }
            }

            Sequental::Unary { dst, op, operand } => {
                let result_ptr = env.get_or_add(dst);
                let operand = atom(env, builder, operand);

                match op {
                    UnOp::Neg(Type::Real) => {
                        writeln!(builder, "pxor xmm0, xmm0")?;
                        writeln!(builder, "subss xmm0, {operand}")?;
                        writeln!(builder, "movups {result_ptr}, xmm0")?;
                    }

                    UnOp::Neg(Type::Int) => {
                        writeln!(builder, "xor eax, eax")?;
                        writeln!(builder, "sub eax, {operand}")?;
                        writeln!(builder, "mov {result_ptr}, eax")?;
                    }

                    UnOp::Neg(Type::F32x2 | Type::F32x3 | Type::F32x4) => {
                        writeln!(builder, "xorps xmm0, xmm0")?;
                        writeln!(builder, "subps xmm0, {operand}")?;
                    }

                    UnOp::Swizzle(swizzle) => {
                        let comb = swizzle.as_slice().into_iter();
                        let comb_code: usize = comb
                            .enumerate()
                            .map(|(n, &comp)| 4usize.pow(n as u32) * comp as usize)
                            .sum();

                        writeln!(builder, "movups xmm0, {operand}")?;
                        writeln!(builder, "shufps xmm0, xmm0, {comb_code}")?;
                        writeln!(builder, "movups {result_ptr}, xmm0")?;
                    }
                }
            }

            Sequental::Call { dst, fun, args } => {
                let shift = env.stack_size() + args.len() * STACK_ALIGN as usize;

                for (n, (arg, ty)) in args.into_iter().rev().enumerate() {
                    let value = atom(env, builder, arg);
                    let address = env.stack_size() + (n + 1) * STACK_ALIGN as usize;

                    match ty {
                        Type::Real => {
                            writeln!(builder, "movups xmm0, {value}")?;
                            writeln!(builder, "movups [rsp - {address}], xmm0")?;
                        }
                        Type::Int => {
                            writeln!(builder, "mov eax, {value}")?;
                            writeln!(builder, "mov [rsp - {address}], eax")?;
                        }
                        _ => {
                            writeln!(builder, "movups xmm0, {value}")?;
                            writeln!(builder, "movaps [rsp - {address}], xmm0")?;
                        }
                    }
                }

                writeln!(builder, "sub rsp, {shift}")?;
                writeln!(builder, "call {fun}")?;
                writeln!(builder, "add rsp, {shift}")?;

                if let Some(dst) = dst {
                    let result_ptr = env.get_or_add(dst);

                    match dst.ty() {
                        Type::Real => {
                            writeln!(builder, "movups {result_ptr}, xmm0")?;
                        }
                        Type::Int => {
                            writeln!(builder, "mov {result_ptr}, eax")?;
                        }
                        _ => {
                            writeln!(builder, "movaps {result_ptr}, xmm0")?;
                        }
                    }
                }
            }
        }

        if let Some(dst) = dst {
            let dst_address = env.get(dst);

            if let Some(phi_dst_ptr) = add_phi(dst, env, None) {
                let phi_dst_ptr = Address::Stack(phi_dst_ptr);

                match dst.ty() {
                    Type::Real => {
                        writeln!(builder, "movups xmm0, {dst_address}")?;
                        writeln!(builder, "movups {phi_dst_ptr}, xmm0")?;
                    }
                    Type::Int => {
                        writeln!(builder, "mov eax, {dst_address}")?;
                        writeln!(builder, "mov {phi_dst_ptr}, eax")?;
                    }
                    _ => {
                        writeln!(builder, "movaps xmm0, {dst_address}")?;
                        writeln!(builder, "movaps {phi_dst_ptr}, xmm0")?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn add_phi(dst: Id, env: &mut Env, mut address: Option<isize>) -> Option<isize> {
    for phi in env.phis {
        if !phi.branches.iter().any(|&id| id == dst) {
            continue;
        }

        if env.has(phi.dst) {
            address = Some(env.addr(phi.dst));
            continue;
        }

        match address {
            Some(address) => {
                env.set(phi.dst, address);
            }
            None => {
                env.get_or_add(phi.dst);
                address = Some(env.addr(phi.dst))
            }
        }

        add_phi(phi.dst, env, address);
    }

    address
}

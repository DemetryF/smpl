use std::fmt;
use std::fmt::Write;

use smplc_lir::{BinOp, Id, Sequental, Type, UnOp};

use crate::{
    builder::Builder,
    compile::atom,
    env::{Address, Env},
    STACK_ALIGN,
};

use super::Compile;

impl Compile for Sequental {
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
                    Type::Real => {
                        writeln!(builder, "movss xmm0, {value}")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")?;
                    }
                    Type::Int => {
                        writeln!(builder, "mov eax, {value}")?;
                        writeln!(builder, "mov {result_ptr}, eax")?;
                    }
                    Type::Complex | Type::Vec2 => {
                        writeln!(builder, "movlps xmm0, {value}")?;
                        writeln!(builder, "movlps {result_ptr}, xmm0")?;
                    }
                    Type::Vec3 | Type::Vec4 => {
                        writeln!(builder, "movaps xmm0, {value}")?;
                        writeln!(builder, "movaps {result_ptr}, xmm0")?;
                    }
                }
            }

            Sequental::Binary {
                dst,
                op,
                ty: Type::Complex,
                lhs,
                rhs,
            } => {
                let result_ptr = env.get_or_add(dst);

                let lhs = atom(env, builder, lhs);
                let rhs = atom(env, builder, rhs);

                writeln!(builder, "movlps xmm0, {lhs}")?;
                writeln!(builder, "movlps xmm1, {rhs}")?;

                match op {
                    BinOp::Add => {
                        writeln!(builder, "addps xmm0, xmm1")?;
                    }
                    BinOp::Sub => {
                        writeln!(builder, "subps xmm0, xmm1")?;
                    }
                    BinOp::Mul => {
                        writeln!(builder, "shufps xmm0, xmm0, 0b00_01_01_00")?; // xmm0 = [ a, b, b, a ]
                        writeln!(builder, "shufps xmm1, xmm1, 0b00_01_00_01")?; // xmm1 = [ c, d, c, d ]
                        writeln!(builder, "mulps  xmm1, xmm1")?; // xmm1 = [ac, bd, bc, ad]
                        writeln!(builder, "movaps xmm0, xmm1")?; // xmm0 = [ac, bd, bc, ad]
                        writeln!(builder, "haddps xmm1, xmm1")?; // xmm1 = [ac - bd, bc - ad]
                        writeln!(builder, "hsubps xmm0, xmm0")?; // xmm0 = [ac + bd, bc + ad]
                        writeln!(builder, "movss  xmm0, xmm1")?; // xmm0 = [ac - bd, bc + ad]
                    }
                    BinOp::Div => {
                        writeln!(builder, "movlps xmm2, xmm1")?; // xmm2 = [ c,  d  ]
                        writeln!(builder, "mulps  xmm2, xmm2")?; // xmm2 = [ cc, dd ]
                        writeln!(builder, "haddps xmm2, xmm2")?; // xmm2 = [ cc + dd ]
                        writeln!(builder, "shufps xmm0, xmm0, 0b00_01_01_00")?; // xmm0 = [ a, b, b, a ]
                        writeln!(builder, "shufps xmm1, xmm1, 0b00_01_00_01")?; // xmm1 = [ c, d, c, d ]
                        writeln!(builder, "mulps  xmm1, xmm1")?; // xmm1 = [ac, bd, bc, ad ]
                        writeln!(builder, "movaps xmm0, xmm1")?; // xmm0 = [ac, bd, bc, ad ]
                        writeln!(builder, "haddps xmm1, xmm1")?; // xmm1 = [ac + bd, bc + ad]
                        writeln!(builder, "hsubps xmm0, xmm0")?; // xmm0 = [ac - bd, bc - ad]
                        writeln!(builder, "movss  xmm0, xmm1")?; // xmm0 = [ac + bd, bc - ad]
                        writeln!(builder, "divps  xmm0, xmm2")?; // xmm0 = [(ac + bd)/(cc + dd), (bc - ad)/(cc + dd)]
                    }
                }

                writeln!(builder, "movss {result_ptr}, xmm0")?;
            }

            Sequental::Binary {
                dst,
                op,
                ty: Type::Vec2 | Type::Vec3 | Type::Vec4,
                lhs,
                rhs,
            } => {
                let result_ptr = env.get_or_add(dst);

                let lhs = atom(env, builder, lhs);
                let rhs = atom(env, builder, rhs);

                writeln!(builder, "movaps xmm0, {lhs}")?;
                writeln!(builder, "movaps xmm1, {rhs}")?;

                match op {
                    BinOp::Add => {
                        writeln!(builder, "addps xmm0, xmm1")?;
                    }
                    BinOp::Sub => {
                        writeln!(builder, "subps xmm0, xmm1")?;
                    }
                    BinOp::Mul => {
                        writeln!(builder, "shufps xmm1, xmm1, 0b00_00_00_00")?;
                        writeln!(builder, "mulps xmm0, xmm1")?;
                    }
                    BinOp::Div => {
                        writeln!(builder, "shufps xmm1, xmm1, 0b00_00_00_00")?;
                        writeln!(builder, "divps xmm0, xmm1")?;
                    }
                }

                writeln!(builder, "movaps {result_ptr}, xmm0")?;
            }

            Sequental::Binary {
                dst,
                op,
                ty,
                lhs,
                rhs,
            } => {
                let result_ptr = env.get_or_add(dst);

                let instruction = match op {
                    BinOp::Add => "add",
                    BinOp::Sub => "sub",
                    BinOp::Mul if ty == Type::Int => "imul",
                    BinOp::Mul => "mul",
                    BinOp::Div => "div",
                };

                let lhs = atom(env, builder, lhs);
                let rhs = atom(env, builder, rhs);

                match ty {
                    Type::Real => {
                        writeln!(builder, "movss xmm0, {lhs}")?;
                        writeln!(builder, "{instruction}ss xmm0, {rhs}")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")?;
                    }
                    Type::Int if op == BinOp::Div => {
                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "mov ebx, {rhs}")?;
                        writeln!(builder, "idiv ebx")?;
                        writeln!(builder, "mov {result_ptr}, eax")?;
                    }
                    Type::Int => {
                        // writeln!(builder, "mov eax, {lhs}")?;
                        // writeln!(builder, "mov ebx, {rhs}")?;
                        // writeln!(builder, "{instruction} eax, ebx")?;
                        // writeln!(builder, "mov {result_ptr}, eax")?;
                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "{instruction} eax, {rhs}")?;
                        writeln!(builder, "mov {result_ptr}, eax")?;
                    }
                    _ => unreachable!(),
                }
            }

            Sequental::Unary {
                dst,
                op,
                ty,
                operand,
            } => {
                let result_ptr = env.get_or_add(dst);
                let operand = atom(env, builder, operand);

                match (ty, op) {
                    (Type::Real, UnOp::Neg) => {
                        writeln!(builder, "pxor xmm0, xmm0")?;
                        writeln!(builder, "subss xmm0, {operand}")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")?;
                    }
                    (Type::Int, UnOp::Neg) => {
                        writeln!(builder, "xor eax, eax")?;
                        writeln!(builder, "sub eax, {operand}")?;
                        writeln!(builder, "mov {result_ptr}, eax")?;
                    }
                    _ => {
                        writeln!(builder, "xorps xmm0, xmm0")?;
                        writeln!(builder, "subps xmm0, {operand}")?;
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
                            writeln!(builder, "movss xmm0, {value}")?;
                            writeln!(builder, "movss [rsp - {address}], xmm0")?;
                        }
                        Type::Int => {
                            writeln!(builder, "mov eax, {value}")?;
                            writeln!(builder, "mov [rsp - {address}], eax")?;
                        }
                        _ => {
                            writeln!(builder, "movaps xmm0, {value}")?;
                            writeln!(builder, "movaps [rsp - {address}], xmm0")?;
                        }
                    }
                }

                writeln!(builder, "sub rsp, {shift}")?;
                writeln!(builder, "call {}", env.functions[&fun])?;
                writeln!(builder, "add rsp, {shift}")?;

                if let Some(dst) = dst {
                    let result_ptr = env.get_or_add(dst);

                    match dst.ty() {
                        Type::Real => {
                            writeln!(builder, "movss {result_ptr}, xmm0")?;
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
                        writeln!(builder, "movss xmm0, {dst_address}")?;
                        writeln!(builder, "movss {phi_dst_ptr}, xmm0")?;
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

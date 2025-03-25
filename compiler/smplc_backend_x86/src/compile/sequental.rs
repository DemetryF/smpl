use std::fmt;
use std::fmt::Write;

use smplc_lir::{BinOp, Id, Sequental, Type, UnOp};

use crate::{
    builder::Builder,
    compile::to_asm,
    env::{address2str, Env},
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

                let value = to_asm(env, builder, value);

                match dst.ty() {
                    Type::Real => {
                        writeln!(builder, "movss xmm0, {value}")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")?;
                    }
                    Type::Int => {
                        writeln!(builder, "mov eax, {value}")?;
                        writeln!(builder, "mov {result_ptr}, eax")?;
                    }
                }
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

                let lhs = to_asm(env, builder, lhs);
                let rhs = to_asm(env, builder, rhs);

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
                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "mov ebx, {rhs}")?;
                        writeln!(builder, "{instruction} eax, ebx")?;
                        writeln!(builder, "mov {result_ptr}, eax")?;
                    }
                }
            }

            Sequental::Unary {
                dst,
                op,
                ty,
                operand,
            } => {
                let result_ptr = env.get_or_add(dst);
                let operand = to_asm(env, builder, operand);

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
                }
            }

            Sequental::Call { dst, fun, args } => {
                let shift = env.stack_size() + args.len() * 8;

                for (n, (arg, ty)) in args.into_iter().rev().enumerate() {
                    let value = to_asm(env, builder, arg);
                    let address = env.stack_size() + (n + 1) * 8;

                    match ty {
                        Type::Real => {
                            writeln!(builder, "movss xmm0, {value}")?;
                            writeln!(builder, "movss DWORD [rsp - {address}], xmm0")?;
                        }
                        Type::Int => {
                            writeln!(builder, "mov eax, {value}")?;
                            writeln!(builder, "mov DWORD [rsp - {address}], eax")?;
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
                    }
                }
            }
        }

        if let Some(dst) = dst {
            let dst_address = env.get(dst);

            if let Some(phi_dst_ptr) = add_phi(dst, env, None) {
                match dst.ty() {
                    Type::Real => {
                        writeln!(builder, "movss xmm0, {dst_address}")?;
                        writeln!(builder, "movss {phi_dst_ptr}, xmm0")?;
                    }
                    Type::Int => {
                        writeln!(builder, "mov eax, {dst_address}")?;
                        writeln!(builder, "mov {phi_dst_ptr}, eax")?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn add_phi(dst: Id, env: &mut Env, mut address: Option<isize>) -> Option<String> {
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

    address.map(address2str)
}

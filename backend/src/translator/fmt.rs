use std::fmt::Display;

use super::instruction::*;

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign(Assign { what, op, to }) => {
                write!(f, "\t{to} {op} {what}")
            }

            Self::Binary(Binary {
                result,
                left,
                op,
                right,
            }) => write!(f, "\t{result} = {left} {op} {right}"),

            Self::Unary(Unary {
                result,
                op,
                operand,
            }) => write!(f, "\t{result} = {op} {operand}"),

            Self::Call(Call {
                result,
                name,
                args_count,
            }) => write!(f, "\t{result} = call {name}, {args_count}"),

            Self::Goto(Goto { to }) => write!(f, "\tgoto {}", to.0),
            Self::Unless(Unless { cond, to }) => write!(f, "\tunless {cond} goto {}", to.0),
            Self::Label(label) => write!(f, "\n{}:", label.0),
            Self::Pop(Pop(pop)) => write!(f, "\tpop {pop}"),
            Self::Push(Push(push)) => write!(f, "\tpush {push}"),
            Self::Return(Return(ret)) => match ret {
                Some(ret) => write!(f, "\treturn {ret}"),
                None => write!(f, "\treturn"),
            },
        }
    }
}

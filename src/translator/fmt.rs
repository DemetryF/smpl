use std::fmt::Display;

use crate::{
    lexer::token::token_value::Literal,
    parser::ast::{expr::Atom, Id},
};

use super::instruction::Instruction;

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign { what, op, to } => {
                write!(f, "\t{} {} {}", to, op, what)
            }
            Self::Binary {
                result,
                left,
                op,
                right,
            } => write!(f, "\t{} = {} {} {}", result, left, op, right),

            Self::Unary {
                result,
                op,
                operand,
            } => write!(f, "\t{} = {} {}", result, op, operand),

            Self::Call {
                result,
                name,
                args_count,
            } => write!(f, "\t{} = call {}, {}", result, name, args_count),

            Self::Goto { to } => write!(f, "\tgoto {}", to.0),
            Self::Unless { cond, to } => write!(f, "\tunless {} goto {}", cond, to.0),
            Self::Label(label) => write!(f, "\n{}:", label.0),
            Self::Pop(pop) => write!(f, "\tpop {}", pop),
            Self::Push(push) => write!(f, "\tpush {}", push),
            Self::Return(ret) => match ret {
                Some(ret) => write!(f, "\treturn {}", ret),
                None => write!(f, "\treturn"),
            },
        }
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{}", id),
            Self::Temp(id) => write!(f, "%{}", id),
            Self::Literal(literal) => write!(f, "{}", literal),
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Bool(bool) => write!(f, "{}", bool),
        }
    }
}

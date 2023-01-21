use std::fmt::Display;

use crate::{lexer::token::token_value::Literal, parser::ast::expr::Atom};

use super::instruction::Instruction;

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign { what, to } => write!(f, "{} = {}", to, what),
            Self::Binary {
                result,
                left,
                op,
                right,
            } => write!(
                f,
                "{} = {} {} {}",
                result,
                left,
                String::from(op.to_owned()),
                right
            ),

            Self::Unary {
                result,
                op,
                operand,
            } => write!(
                f,
                "{} = {} {}",
                result,
                String::from(op.to_owned()),
                operand
            ),

            Self::Call {
                result,
                name,
                args_count,
            } => write!(f, "{} = call {}, {}", result, name, args_count),

            Self::Goto { to } => write!(f, "goto {}", to.0),
            Self::Unless { cond, to } => write!(f, "unless {} goto {}", cond, to.0),
            Self::Label(label) => write!(f, "{}:", label.0),
            Self::Pop(pop) => write!(f, "pop {}", pop),
            Self::Push(push) => write!(f, "push {}", push),
            Self::Return(ret) => match ret {
                Some(ret) => write!(f, "return {}", ret),
                None => write!(f, "return"),
            },
        }
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{}", id),
            Self::Literal(literal) => write!(f, "{}", literal),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num.to_string()),
            Self::Bool(bool) => write!(f, "{}", bool.to_string()),
        }
    }
}

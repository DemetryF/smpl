use crate::{lexer::token::operator::Operator, parser::ast::expr::Atom};

#[derive(Clone, Debug)]
pub struct Label(pub String);

#[derive(Debug)]
pub enum Instruction {
    Binary {
        result: Atom,
        left: Atom,
        op: Operator,
        right: Atom,
    },
    Unary {
        result: Atom,
        op: Operator,
        operand: Atom,
    },
    Assign {
        what: Atom,
        op: Operator,
        to: Atom,
    },
    Goto {
        to: Label,
    },
    Unless {
        cond: Atom,
        to: Label,
    },
    Call {
        result: Atom,
        name: String,
        args_count: usize,
    },

    Label(Label),
    Return(Option<Atom>),
    Push(Atom),
    Pop(String),
}

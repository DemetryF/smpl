use crate::{lexer::token::operator::Operator, parser::ast::expr::Atom};

#[derive(Clone, Debug)]
pub struct Label(pub String);

#[derive(Debug)]
pub enum Instruction {
    Binary {
        result: String,
        left: Atom,
        op: Operator,
        right: Atom,
    },
    Unary {
        result: String,
        op: Operator,
        operand: Atom,
    },
    Assign {
        what: Atom,
        to: String,
    },
    Goto {
        to: Label,
    },
    IfFalse {
        cond: Atom,
        to: Label,
    },
    Call {
        result: String,
        name: String,
        args_count: usize,
    },

    Label(Label),
    Return(Option<Atom>),
    Push(Atom),
    Pop(String),
}

mod display;

use smplc_macros::{display, EnumWrap};

pub use smplc_hir::{ArithmOp, NumberType, RelOp, Type};

use crate::{Atom, FunctionId, Id, Label};

#[derive(EnumWrap)]
pub enum Instruction {
    Assign(Assign),

    If(If),
    Goto(Goto),
    Call(Call),

    Return(Return),
    Halt(Halt),
}

pub struct Assign {
    pub lhs: Id,
    pub rhs: AssignRhs,
}

pub enum AssignRhs {
    Binary {
        lhs: Atom,
        op: ArithmOp,
        rhs: Atom,
        ty: NumberType,
    },
    Neg {
        rhs: Atom,
        ty: NumberType,
    },
    Call(Call, NumberType),
    Atom(Atom),
}

pub struct If {
    pub lhs: Atom,
    pub op: RelOp,
    pub ty: NumberType,
    pub rhs: Atom,

    pub then_label: Option<Label>,
    pub else_label: Option<Label>,
}

#[display("goto {label}")]
pub struct Goto {
    pub label: Label,
}

pub struct Call {
    pub id: FunctionId,
    pub args: Vec<(Atom, NumberType)>,
}

#[display("halt")]
pub struct Halt;

pub struct Return {
    pub value: Option<Atom>,
}

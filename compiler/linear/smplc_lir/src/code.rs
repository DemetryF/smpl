mod display;

use std::collections::HashMap;

use smplc_hir::{self as hir};

use crate::{FunctionId, Id, Instruction, Label};

#[derive(Default)]
pub struct Code {
    pub functions: Vec<CodeFunction>,
    pub constants: HashMap<Id, Number>,
}

pub struct CodeFunction {
    pub id: FunctionId,
    pub args: Vec<Id>,
    pub instructions: Vec<Instruction>,
    pub labels: HashMap<usize, Label>,
}

impl CodeFunction {
    pub fn new(id: FunctionId, args: Vec<Id>) -> Self {
        Self {
            id,
            args,
            instructions: Default::default(),
            labels: Default::default(),
        }
    }

    pub fn push(&mut self, instruction: impl Into<Instruction>) {
        self.instructions.push(instruction.into());
    }

    pub fn add_label(&mut self, label: Label) {
        self.labels.insert(self.instructions.len(), label);
    }
}

impl Code {
    pub fn push(&mut self, instruction: impl Into<Instruction>) {
        self.functions.last_mut().unwrap().push(instruction);
    }

    pub fn add_function(&mut self, function: CodeFunction) {
        self.functions.push(function);
    }

    pub fn add_label(&mut self, label: Label) {
        self.functions.last_mut().unwrap().add_label(label);
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Number {
    Real(f32),
    Int(i32),
}

impl Number {
    pub fn real(self) -> f32 {
        let Self::Real(value) = self else { panic!() };

        value
    }

    pub fn int(self) -> i32 {
        let Self::Int(value) = self else { panic!() };

        value
    }
}

impl From<hir::Literal> for Number {
    fn from(value: hir::Literal) -> Self {
        match value {
            hir::Literal::Real(num) => Self::Real(num),
            hir::Literal::Int(num) => Self::Int(num),
            hir::Literal::Bool(bool) => Self::Int(bool as i32),
        }
    }
}

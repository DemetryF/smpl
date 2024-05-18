mod display;

use std::collections::HashMap;

use crate::{FunctionId, Id, Instruction, Label};

#[derive(Default)]
pub struct Code {
    pub functions: Vec<CodeFunction>,
    pub constants: HashMap<Id, f32>,
}

#[derive(Default)]
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
            ..Default::default()
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

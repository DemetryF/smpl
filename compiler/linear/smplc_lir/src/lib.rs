pub mod instruction;

use std::collections::HashMap;

pub use instruction::*;

pub struct LIR {
    pub functions: HashMap<FunId, CodeFunction>,
    pub function_names: HashMap<FunId, String>,
    pub constants: HashMap<Id, Number>,
    pub labels: HashMap<Label, String>,
}

#[derive(Clone, Copy)]
pub enum Number {
    Real(f32),
    Int(i32),
}

pub struct CodeFunction {
    pub args: Vec<Id>,
    pub code: Code,
}

pub struct Code {
    pub blocks: Vec<BasicBlock>,
    pub phis: Vec<Phi>,
}

impl Code {
    pub fn push(&mut self, instr: impl Into<Instruction>) {
        match instr.into() {
            Instruction::ControlFlow(instr) => {
                self.blocks.last_mut().unwrap().end = Some(instr);
                self.blocks.push(Default::default());
            }

            Instruction::Phi(phi) => {
                self.phis.push(phi);
            }

            Instruction::Sequental(instr) => {
                self.blocks.last_mut().unwrap().instructions.push(instr);
            }
        }
    }

    pub fn label(&mut self, label: Label) {
        self.blocks.push(BasicBlock::with_label(label));

        let last_block = self.blocks.last_mut().unwrap();

        if last_block.label.is_none() && last_block.is_empty() {
            last_block.label = Some(label);
        }
    }
}

#[derive(Default)]
pub struct BasicBlock {
    pub label: Option<Label>,
    pub instructions: Vec<Sequental>,
    pub end: Option<ControlFlow>,
}

impl BasicBlock {
    pub fn with_label(label: Label) -> Self {
        Self {
            label: Some(label),
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.end.is_none() && self.instructions.is_empty()
    }
}

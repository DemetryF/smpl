pub mod instruction;

mod display;
mod value;

use std::collections::{BTreeMap, HashMap};

pub use instruction::*;
pub use value::*;

pub struct LIR<'f> {
    pub bodies: BTreeMap<FunId<'f>, FunctionBody<'f>>,
    pub constants: HashMap<Id, Value>,
    pub labels: HashMap<Label, String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunId<'f>(pub &'f str);

pub struct FunctionBody<'f> {
    pub args: Vec<Id>,
    pub code: Code<'f>,
}

#[derive(Default)]
pub struct Code<'f> {
    pub blocks: Vec<BasicBlock<'f>>,
    pub phis: Vec<Phi>,
}

impl<'f> Code<'f> {
    pub fn push(&mut self, instr: impl Into<Instruction<'f>>) {
        match instr.into() {
            Instruction::ControlFlow(instr) => {
                if self.blocks.is_empty() {
                    self.blocks.push(Default::default());
                }

                self.blocks.last_mut().unwrap().end = Some(instr);
                self.blocks.push(Default::default());
            }

            Instruction::Phi(phi) => {
                self.phis.push(phi);
            }

            Instruction::Sequental(instr) => {
                if self.blocks.is_empty() {
                    self.blocks.push(Default::default());
                }

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

    pub fn append(&mut self, mut other: Self) {
        self.blocks.append(&mut other.blocks);
        self.phis.append(&mut other.phis);
    }
}

#[derive(Default)]
pub struct BasicBlock<'f> {
    pub label: Option<Label>,
    pub instructions: Vec<Sequental<'f>>,
    pub end: Option<ControlFlow>,
}

impl BasicBlock<'_> {
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

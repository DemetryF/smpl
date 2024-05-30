use std::collections::LinkedList;

use crate::{instruction::Instruction, Label};

#[derive(Default)]
pub struct Code {
    blocks: Vec<BasicBlock>,
}

impl Code {
    pub fn push_instr(&mut self, instr: Instruction) {
        if matches!(instr, Instruction::Goto(_) | Instruction::IfRel { .. }) {
            self.new_block()
        }

        self.last_block().push(instr)
    }

    pub fn push_label(&mut self, label: Label) {
        if self.last_block().is_empty() {
            self.last_block().label = Some(label);
        } else {
            self.blocks.push(BasicBlock::with_label(label));
        }
    }

    fn last_block(&mut self) -> &mut BasicBlock {
        if self.blocks.is_empty() {
            self.new_block()
        }

        self.blocks.last_mut().unwrap()
    }

    fn new_block(&mut self) {
        if self.blocks.is_empty() || !self.last_block().is_empty() {
            self.blocks.push(BasicBlock::default())
        }
    }
}

#[derive(Default)]
pub struct BasicBlock {
    label: Option<Label>,
    instructions: Instructions,
}

impl BasicBlock {
    pub fn with_label(label: Label) -> Self {
        Self {
            label: Some(label),
            ..Default::default()
        }
    }

    pub fn push(&mut self, instr: Instruction) {
        self.instructions.push(instr)
    }

    pub fn is_empty(&self) -> bool {
        self.label.is_none() || self.instructions.is_empty()
    }
}

#[derive(Default)]
pub struct Instructions {
    data: LinkedList<Instruction>,
}

impl Instructions {
    pub fn push(&mut self, instr: Instruction) {
        self.data.push_back(instr.into())
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

use std::collections::{linked_list, LinkedList};

use crate::{instruction::Instruction, Label};

#[derive(Default)]
pub struct Code {
    pub blocks: Vec<BasicBlock>,
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
    pub label: Option<Label>,
    pub instructions: Instructions,
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

    pub fn tail_jump_dst(&self) -> Option<&Label> {
        match self.instructions.last() {
            Some(Instruction::IfRel { label, .. }) => Some(label),
            Some(Instruction::Goto(label)) => Some(label),

            _ => None,
        }
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

    pub fn iter(&self) -> linked_list::Iter<Instruction> {
        self.data.iter()
    }

    pub fn last(&self) -> Option<&Instruction> {
        self.data.back()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl IntoIterator for Instructions {
    type Item = Instruction;

    type IntoIter = linked_list::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

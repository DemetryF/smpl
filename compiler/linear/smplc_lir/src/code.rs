use std::collections::{linked_list, LinkedList};

use petgraph::Graph;

use crate::{instruction::Instruction, Label};

pub type Code = Graph<BasicBlock, ()>;

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
        self.label.is_none() && self.instructions.is_empty()
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

    pub fn append(&mut self, other: &mut Instructions) {
        self.data.append(&mut other.data)
    }
}

impl IntoIterator for Instructions {
    type Item = Instruction;

    type IntoIter = linked_list::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

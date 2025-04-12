pub mod instruction;

mod display;
mod op;
mod value;

use std::{
    cmp,
    collections::{BTreeMap, HashMap},
};

pub use display::*;
pub use instruction::*;
pub use op::*;
pub use value::*;

pub struct LIR<'f> {
    pub bodies: BTreeMap<FunId<'f>, FunctionBody<'f>>,
    pub constants: HashMap<Id, Value>,
    pub labels: HashMap<Label, String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FunId<'f> {
    pub name: &'f str,
    ret_ty: Option<Type>,
}

impl<'f> FunId<'f> {
    pub fn new(name: &'f str, ret_ty: Option<Type>) -> Self {
        Self { name, ret_ty }
    }

    pub fn ret_ty(self) -> Option<Type> {
        self.ret_ty
    }
}

impl PartialOrd for FunId<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for FunId<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name.cmp(other.name)
    }
}

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

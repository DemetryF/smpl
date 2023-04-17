use std::collections::HashMap;

use crate::instruction::{Instruction, Label};

#[derive(Default)]
pub struct Code {
    instructions: Vec<Instruction>,
    labels: HashMap<usize, Label>,
}

impl Code {
    pub fn push(&mut self, instruction: impl Into<Instruction>) {
        self.instructions.push(instruction.into())
    }

    pub fn pop(&mut self) -> Instruction {
        self.instructions.pop().unwrap()
    }

    pub fn add_label(&mut self, label: Label) {
        self.labels.insert(self.instructions.len(), label);
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, instruction) in self.instructions.iter().enumerate() {
            if let Some(label) = self.labels.get(&i) {
                writeln!(f, "\n{label}:")?;
            }

            writeln!(f, "\t{instruction}")?;
        }

        writeln!(f)
    }
}

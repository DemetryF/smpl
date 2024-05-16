use std::collections::HashMap;
use std::fmt;

pub use smplc_ast::{BinOp, UnOp};

pub use atom::Atom;
pub use id::{FunctionId, Id};
pub use instructions::*;

mod atom;
mod display;
mod id;
mod instructions;

#[derive(Clone)]
pub struct Label(pub String);

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

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for function in self.functions.iter() {
            let id = &function.id;

            write!(f, "\n{id}")?;

            write!(f, "(")?;

            for (i, arg) in function.args.iter().enumerate() {
                write!(f, "{arg}")?;

                if i + 1 != function.args.len() {
                    write!(f, ", ")?;
                }
            }

            writeln!(f, "):")?;

            for (index, instruction) in function.instructions.iter().enumerate() {
                if let Some(label) = function.labels.get(&index) {
                    writeln!(f, "\n    {label}:")?;
                }

                writeln!(f, "        {instruction}")?;
            }

            if let Some(label) = function.labels.get(&function.instructions.len()) {
                writeln!(f, "\n    {label}:")?;
            }
        }

        writeln!(f)
    }
}

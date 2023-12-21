use std::collections::HashMap;

pub use instruction::*;

mod instruction;

#[derive(Default)]
pub struct Code {
    pub functions: Vec<CodeFunction>,
}

pub struct CodeFunction {
    pub id: String,
    pub args: Vec<Id>,
    pub instructions: Vec<Instruction>,
    pub labels: HashMap<usize, Label>,
}

impl CodeFunction {
    pub fn new(id: String, args: Vec<Id>) -> Self {
        Self {
            id,
            args,
            instructions: Vec::new(),
            labels: HashMap::new(),
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

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for function in self.functions.iter() {
            let id = &function.id;
            let args = function
                .args
                .iter()
                .map(|arg| arg.0.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(f, "\n{}({}):", id, args)?;

            for (index, instruction) in function.instructions.iter().enumerate() {
                if let Some(label) = function.labels.get(&index) {
                    writeln!(f, "\n    {label}:")?;
                }

                writeln!(f, "        {instruction}")?;
            }
        }

        writeln!(f)
    }
}

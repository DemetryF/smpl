use std::collections::HashMap;

pub use instruction::*;

mod instruction;

#[derive(Default)]
pub struct Code {
    pub functions: Vec<CodeFunction>,
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

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for function in self.functions.iter() {
            let id = &function.id;

            writeln!(f, "\n{id}")?;

            write!(f, "(")?;

            function
                .args
                .iter()
                .try_for_each(|arg| write!(f, "{arg}"))?;

            write!(f, ")")?;

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

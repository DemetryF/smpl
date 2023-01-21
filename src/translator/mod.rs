use crate::parser::Parser;

use self::{instruction::Instruction, translate::Translate};

pub mod fmt;
pub mod instruction;
pub mod translate;

pub struct Translator {
    pub instructions: Vec<Instruction>,
    parser: Parser,

    temps_count: usize,
    pub ifs_count: usize,
    pub whiles_count: usize,
}

impl Translator {
    pub fn new(code: String) -> Self {
        Self {
            instructions: Vec::new(),
            parser: Parser::new(code),

            temps_count: 0,
            ifs_count: 0,
            whiles_count: 0,
        }
    }

    pub fn get_temp_var(&mut self) -> String {
        self.temps_count += 1;
        return String::from("t") + self.temps_count.to_string().as_str();
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn translate(&mut self) {
        self.parser.parse().translate(self);
    }
}

use crate::parser::{ast::statement::Statement, Parser};

use self::{
    instruction::{Instruction, Label},
    translate::Translate,
};

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
        let mut stmts = self.parser.parse();
        let mut no_funcs = Vec::new();

        for stmt in stmts {
            if let Statement::Function(func) = stmt {
                func.translate(self);
            } else {
                no_funcs.push(stmt);
            }
        }

        self.push(Instruction::Label(Label(String::from("main"))));

        for stmt in no_funcs {
            stmt.translate(self);
        }
    }
}

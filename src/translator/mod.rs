use crate::{
    ast::{Atom, Statement},
    error::*,
    parser::Parser,
    static_analyzer::StaticAnalyzer,
};

use self::{
    instruction::{Instruction, Label},
    translate::Translate,
};

pub mod fmt;
pub mod instruction;
pub mod translate;

pub struct Translator<'code> {
    pub instructions: Vec<Instruction>,
    parser: Parser<'code>,

    temps_count: usize,
    pub ifs_count: usize,
    pub whiles_count: usize,
}

impl<'code> Translator<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            instructions: Vec::new(),
            parser: Parser::new(code),

            temps_count: 0,
            ifs_count: 0,
            whiles_count: 0,
        }
    }

    pub fn get_temp_var(&mut self) -> Atom {
        self.temps_count += 1;
        Atom::Temp(self.temps_count)
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn translate(&mut self) -> std::result::Result<(), Vec<Error>> {
        let stmts = self.parser.parse().map_err(|e| vec![e])?;
        let analyzer = StaticAnalyzer::new(&stmts);

        if !analyzer.errors.is_empty() {
            return Err(analyzer.errors);
        }

        let (global, local) = Self::global_and_local_stmts(stmts);

        self.translate_stmts(global);
        self.add_main_label();
        self.translate_stmts(local);

        Ok(())
    }

    pub fn add_main_label(&mut self) {
        self.push(Instruction::Label(Label("main".into())));
    }

    pub fn translate_stmts(&mut self, stmts: Vec<Statement>) {
        for stmt in stmts {
            stmt.translate(self);
        }
    }

    pub fn global_and_local_stmts(stmts: Vec<Statement>) -> (Vec<Statement>, Vec<Statement>) {
        let mut global_stmts = Vec::new();
        let mut local_stmts = Vec::new();

        for stmt in stmts {
            match stmt {
                Statement::Function(_) => global_stmts.push(stmt),
                stmt => local_stmts.push(stmt),
            }
        }

        (global_stmts, local_stmts)
    }
}

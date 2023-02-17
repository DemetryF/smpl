use enum_dispatch::enum_dispatch;

use crate::translator::instruction::{
    Assign, Binary, Call, Goto, Instruction, Label, Pop, Push, Return, Unary, Unless,
};

pub struct Compiler {
    pub code: String,
}

impl Compiler {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            code: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn compile(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            instruction.compile(self);
        }
    }
}

#[enum_dispatch(Instruction)]
pub trait Compile {
    fn compile(self, _compiler: &mut Compiler);
}

impl Compile for Binary {
    fn compile(self, _compiler: &mut Compiler) {
        todo!()
    }
}
impl Compile for Unary {
    fn compile(self, _compiler: &mut Compiler) {
        todo!()
    }
}
impl Compile for Assign {
    fn compile(self, _compiler: &mut Compiler) {
        todo!()
    }
}
impl Compile for Goto {
    fn compile(self, _compiler: &mut Compiler) {
        todo!()
    }
}
impl Compile for Unless {
    fn compile(self, _compiler: &mut Compiler) {
        todo!()
    }
}
impl Compile for Call {
    fn compile(self, _compiler: &mut Compiler) {
        todo!()
    }
}
impl Compile for Label {
    fn compile(self, _compiler: &mut Compiler) {
        todo!()
    }
}
impl Compile for Return {
    fn compile(self, _compiler: &mut Compiler) {
        todo!()
    }
}
impl Compile for Push {
    fn compile(self, _compiler: &mut Compiler) {
        todo!()
    }
}
impl Compile for Pop {
    fn compile(self, _compiler: &mut Compiler) {
        todo!()
    }
}

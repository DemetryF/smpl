pub mod builder;
mod code;
mod instruction;

use std::collections::HashMap;

pub use code::*;
pub use instruction::*;

pub struct LIR {
    pub functions: Vec<Function>,
    pub constants: HashMap<Id, Value>,
    pub externs: Vec<FnExtern>,
}

pub struct Function {
    pub id: FnId,
    pub args: Vec<Type>,
    pub code: Code,
}

pub struct FnExtern {
    pub id: FnId,
    pub args: Vec<Type>,
}

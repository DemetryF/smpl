use std::fmt::{self, Write};

use crate::env::Address;

#[derive(Default)]
pub struct Builder {
    code: String,

    constants: Vec<SimdConstant>,
}

impl Write for Builder {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.code += s;

        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.code.push(c);

        Ok(())
    }
}

impl Builder {
    pub fn build(mut self) -> Result<String, fmt::Error> {
        writeln!(
            self,
            "
section .data
    fmtr:     db \"%f\", 10, 0
    fmti:     db \"%ld\", 10, 0
    fmttrue:  db \"true\", 10, 0
    fmtfalse: db \"false\", 10, 0
    fmtvec2:  db \"(%f, %f)\", 10, 0
    fmtvec3:  db \"(%f, %f, %f)\", 10, 0
    fmtvec4:  db \"(%f, %f, %f, %f)\", 10, 0
"
        )?;

        if !self.constants.is_empty() {
            for (index, &constant) in self.constants.iter().enumerate() {
                writeln!(self.code, "LC{index}: dd {constant}")?;
            }
        }

        Ok(self.code)
    }

    pub fn constant(&mut self, constant: impl Into<SimdConstant>) -> Address {
        let constant = constant.into();

        let index = match self.constants.iter().position(|&c| c == constant) {
            Some(index) => index,
            None => {
                self.constants.push(constant);

                self.constants.len() - 1
            }
        };

        Address::Const(index)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SimdConstant {
    X1([f32; 1]),
    X2([f32; 2]),
    X3([f32; 3]),
    X4([f32; 4]),
}

impl SimdConstant {
    pub fn slice(&self) -> &[f32] {
        match self {
            SimdConstant::X1(constant) => &constant[..],
            SimdConstant::X2(constant) => &constant[..],
            SimdConstant::X3(constant) => &constant[..],
            SimdConstant::X4(constant) => &constant[..],
        }
    }
}

impl From<[f32; 1]> for SimdConstant {
    fn from(value: [f32; 1]) -> Self {
        Self::X1(value)
    }
}

impl From<[f32; 2]> for SimdConstant {
    fn from(value: [f32; 2]) -> Self {
        Self::X2(value)
    }
}

impl From<[f32; 3]> for SimdConstant {
    fn from(value: [f32; 3]) -> Self {
        Self::X3(value)
    }
}

impl From<[f32; 4]> for SimdConstant {
    fn from(value: [f32; 4]) -> Self {
        Self::X4(value)
    }
}

impl fmt::Display for SimdConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &x in self.slice() {
            write!(f, "{}, ", x.to_bits())?;
        }

        Ok(())
    }
}

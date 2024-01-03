use std::collections::HashMap;
use std::fmt::{self, Write};

use smplc_ir::FunctionId;

#[derive(Default)]
pub struct Builder {
    code: String,
    float_constants: Vec<f32>,
    pub function_arg_sizes: HashMap<FunctionId, usize>,
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
fmt: db \"%f\", 10, 0
"
        )?;

        if !self.float_constants.is_empty() {
            for (index, float) in self.float_constants.iter().enumerate() {
                if float.fract() == 0.0 {
                    writeln!(self.code, "LC{index}: dd {float:.1}")?;
                } else {
                    writeln!(self.code, "LC{index}: dd {float}")?;
                }
            }
        }

        Ok(self.code)
    }

    pub fn float(&mut self, float: f32) -> String {
        fn label(index: usize) -> String {
            format!("dword[LC{index}]")
        }

        let index = match self.float_constants.iter().position(|&f| f == float) {
            Some(index) => index,
            None => {
                self.float_constants.push(float);

                self.float_constants.len() - 1
            }
        };

        label(index)
    }
}

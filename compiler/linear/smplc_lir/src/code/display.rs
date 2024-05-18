use std::fmt;

use crate::Code;

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

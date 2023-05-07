use std::fmt;

use colored::Colorize;
use frontend::ast::Pos;

pub struct Error<'source, K: fmt::Display> {
    pub filename: &'source str,
    pub code: &'source str,

    pub pos: Pos,
    pub kind: K,
}

impl<'source, K: fmt::Display> fmt::Display for Error<'source, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: ",
            self.filename, self.pos.line, self.pos.column
        )?;

        writeln!(f, "{} {}", "Error:".red(), self.kind)?;

        let column_length = self.pos.line.ilog10() as usize + 1;

        write!(f, " {} | ", self.pos.line)?;
        writeln!(f, "{}", self.get_line())?;

        write!(f, " {} | ", " ".repeat(column_length))?;
        write!(f, "{}{}", " ".repeat(self.pos.column - 1), "^".red())
    }
}

impl<'source, K: fmt::Display> Error<'source, K> {
    pub fn get_line(&self) -> &str {
        let line = &self.code[self.pos.line_start..];

        if let Some((line, _)) = line.split_once('\n') {
            line
        } else {
            line
        }
    }
}

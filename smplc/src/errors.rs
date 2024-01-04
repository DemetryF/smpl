use std::fmt;

use colored::Colorize;
use smplc_ast::Pos;

struct Error<'source, K: fmt::Display> {
    pub filename: &'source str,
    pub code: &'source str,

    pub pos: Pos,
    pub kind: K,
}

pub fn output_error<'source>(
    filename: &'source str,
    code: &'source str,
    pos: Pos,
    kind: impl fmt::Display,
) {
    eprintln!(
        "{}",
        Error {
            filename,
            code,
            pos,
            kind,
        }
    );
}

impl<'source, K: fmt::Display> fmt::Display for Error<'source, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: ",
            self.filename,
            self.pos.line(),
            self.pos.column()
        )?;

        writeln!(f, "{} {}", "Error:".red(), self.kind)?;

        let column_length = self.pos.line().ilog10() as usize + 1;

        write!(f, " {} | ", self.pos.line())?;
        writeln!(f, "{}", self.get_line())?;

        write!(f, " {} | ", " ".repeat(column_length))?;
        write!(f, "{}{}", " ".repeat(self.pos.column() - 1), "^".red())
    }
}

impl<'source, K: fmt::Display> Error<'source, K> {
    pub fn get_line(&self) -> &'source str {
        self.code[self.pos.line_start()..]
            .lines()
            .next()
            .unwrap_or_default()
    }
}

use std::fmt;

use colored::Colorize;
use smplc_ast::Span;

struct Error<'source, K: fmt::Display> {
    pub filename: &'source str,
    pub code: &'source str,

    pub span: Span,
    pub kind: K,
}

pub fn output_error<'source>(
    filename: &'source str,
    code: &'source str,
    span: Span,
    kind: impl fmt::Display,
) {
    eprintln!(
        "{}",
        Error {
            filename,
            code,
            span,
            kind,
        }
    );
}

impl<K: fmt::Display> fmt::Display for Error<'_, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: ", self.filename, self.span)?;

        writeln!(f, "{} {}", "Error:".red(), self.kind)?;

        let column_length = self.span.start().line().ilog10() as usize + 1;

        write!(f, " {} | ", self.span.start().line())?;
        writeln!(f, "{}", self.get_line())?;

        write!(f, " {} | ", " ".repeat(column_length))?;
        write!(
            f,
            "{}{}",
            " ".repeat(self.span.start().column() - 1),
            "^".repeat(self.span.len()).red()
        )
    }
}

impl<'source, K: fmt::Display> Error<'source, K> {
    pub fn get_line(&self) -> &'source str {
        self.code[self.span.start().line_start()..]
            .lines()
            .next()
            .unwrap_or_default()
    }
}

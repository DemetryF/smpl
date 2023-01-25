use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
    pub line_begin: usize,
    pub index: usize,
}

impl Pos {
    pub fn empty() -> Self {
        Self {
            line: 1,
            column: 1,
            line_begin: 0,
            index: 0,
        }
    }

    pub fn change(&mut self, char: char) {
        match char {
            '\n' => self.new_line(),
            _ => self.column += 1,
        }

        self.index += 1;
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
        self.line_begin = self.index + 1;
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

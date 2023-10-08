#[derive(Clone, Copy, Debug)]
pub struct Pos {
    line: usize,
    column: usize,
    line_start: usize,
}

impl Pos {
    pub fn line(self) -> usize {
        self.line
    }

    pub fn column(self) -> usize {
        self.column
    }

    pub fn line_start(self) -> usize {
        self.line_start
    }

    pub fn index(self) -> usize {
        self.line_start() + self.column() - 1
    }

    pub(crate) fn update(&mut self, char: char) {
        self.column += 1;

        if char == '\n' {
            self.line += 1;
            self.line_start = self.index();
            self.column = 1;
        }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            line_start: 0,
        }
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

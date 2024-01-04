#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    line: usize,
    column: usize,
    index: usize,
}

impl Pos {
    #[inline(always)]
    pub fn new(line: usize, column: usize, index: usize) -> Self {
        Self {
            line,
            column,
            index,
        }
    }

    #[inline(always)]
    pub fn line(self) -> usize {
        self.line
    }

    #[inline(always)]
    pub fn column(self) -> usize {
        self.column
    }

    #[inline(always)]
    pub fn line_start(self) -> usize {
        self.index - self.column + 1
    }

    #[inline(always)]
    pub fn index(self) -> usize {
        self.index
    }

    #[inline(always)]
    pub fn update(&mut self, char: char) {
        match char {
            '\n' => {
                self.line += 1;
                self.column = 1;
            }

            _ => self.column += 1,
        }

        self.index += 1;
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            index: 0,
        }
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

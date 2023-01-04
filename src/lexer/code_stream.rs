use super::token::pos::Pos;

pub struct CodeStream {
    pub code: String,
    pub pos: Pos,
}

impl CodeStream {
    pub fn new(code: String) -> Self {
        Self {
            code: code,
            pos: Pos::empty(),
        }
    }

    pub fn current(&self) -> char {
        let ch: Option<char> = self.code.chars().nth(self.pos.index);

        match ch {
            None => panic!("panic at CodeStream::current"),
            Some(ch) => ch,
        }
    }

    pub fn check(&self, str: &str) -> bool {
        let start: usize = self.pos.index;
        let end: usize = self.pos.index + str.len();

        if end > self.code.len() {
            return false;
        }

        return &self.code[start..end] == str;
    }

    pub fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.accept();
        }
    }

    pub fn accept(&mut self) -> char {
        self.pos.column += 1;

        if self.check("\n") {
            self.pos.line += 1;
            self.pos.column = 1;
            self.pos.line_begin = self.pos.index + 1;
        }

        let ch: char = self.current();
        self.pos.index += 1;

        ch
    }

    pub fn is_eof(&self) -> bool {
        self.pos.index >= self.code.len()
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }
}

use super::pos::Pos;

pub struct CodeStream<'code> {
    pub pos: Pos,
    code: &'code str,
}

impl<'code> CodeStream<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            code,
            pos: Pos::empty(),
        }
    }

    pub fn current(&self) -> char {
        self.code[self.pos.index..]
            .chars()
            .next()
            .expect("CodeStream::current")
    }

    pub fn check(&self, str: &str) -> bool {
        let start = self.pos.index;
        let end = start + str.len();

        if end > self.code.len() {
            return false;
        }

        self.get_code_slice(start, str.len()) == str
    }

    pub fn get_code_slice(&self, start: usize, len: usize) -> &'code str {
        &self.code[start..start + len]
    }

    pub fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.accept();
        }
    }

    pub fn accept(&mut self) -> char {
        let ch = self.current();
        self.pos.change(ch);

        ch
    }

    pub fn is_eof(&self) -> bool {
        self.pos.index >= self.code.len()
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }
}

use smplc_ast::Pos;

pub struct CodeStream<'source> {
    code: &'source str,
    pos: Pos,
}

impl<'source> CodeStream<'source> {
    pub fn new(code: &'source str) -> Self {
        Self {
            code,
            pos: Pos::default(),
        }
    }

    pub fn current(&self) -> char {
        self.code[self.pos.index()..].chars().next().unwrap()
    }

    pub fn next_ch(&mut self) -> char {
        let ch = self.current();

        self.pos.update(ch);

        ch
    }

    pub fn check(&self, char: char) -> bool {
        !self.is_eof() && self.current() == char
    }

    pub fn check_slice(&self, str: &str) -> bool {
        let start = self.index();
        let end = self.index() + str.len();

        if end > self.code.len() {
            return false;
        }

        self.slice(start, end) == str
    }

    pub fn slice(&self, start: usize, end: usize) -> &'source str {
        self.code.get(start..end).unwrap_or_default()
    }

    pub fn slice_from_current(&self, len: usize) -> &'source str {
        self.slice(self.pos.index(), self.pos.index() + len)
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }

    pub fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.next_ch();
        }
    }

    pub fn index(&self) -> usize {
        self.pos.index()
    }

    pub fn is_eof(&self) -> bool {
        self.pos.index() >= self.code.len()
    }
}

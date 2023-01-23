use crate::lexer::code_stream::CodeStream;

pub struct CommentType {
    pub begin: &'static str,
    pub end: &'static str,
}

impl CommentType {
    pub fn is_begin(&self, code: &CodeStream) -> bool {
        code.check(self.begin)
    }

    fn is_end(&self, code: &CodeStream) -> bool {
        code.check(self.end)
    }

    pub fn try_skip(&self, code: &mut CodeStream) {
        if !self.is_begin(code) {
            return;
        }

        code.skip(self.begin.len());

        while !self.is_end(code) && !code.is_eof() {
            code.accept();
        }

        code.skip(self.end.len());
    }
}

use crate::lexer::CodeStream;

pub struct CommentType {
    pub begin: &'static str,
    pub end: &'static str,
}

impl CommentType {
    pub fn is_begin(&self, code_stream: &CodeStream) -> bool {
        code_stream.check_seq(self.begin)
    }

    fn is_end(&self, code_stream: &CodeStream) -> bool {
        code_stream.check_seq(self.end)
    }

    pub fn try_skip(&self, code_stream: &mut CodeStream) {
        if !self.is_begin(code_stream) {
            return;
        }

        code_stream.skip(self.begin.len());

        while !self.is_end(code_stream) && !code_stream.is_eof() {
            code_stream.consume();
        }

        code_stream.skip(self.end.len());
    }
}

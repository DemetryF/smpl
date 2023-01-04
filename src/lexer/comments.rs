use super::code_stream::CodeStream;

pub struct Comments;

impl Comments {
    pub fn skip(code: &mut CodeStream) {
        Self::skip_spaces(code);

        if Self::is_line_comment_begin(code) {
            Self::line_comment(code);
        } else if Self::is_block_comment_begin(code) {
            Self::block_comment(code);
        }

        Self::skip_spaces(code);

        if Self::is_line_comment_begin(code) || Self::is_block_comment_begin(code) {
            Self::skip(code);
        } else {
            Self::skip_spaces(code);
        }
    }

    fn skip_spaces(code: &mut CodeStream) {
        while !code.is_eof() && code.current().is_ascii_whitespace() {
            code.accept();
        }
    }

    fn line_comment(code: &mut CodeStream) {
        code.skip(2);
        while !Self::is_line_comment_end(code) {
            code.accept();
        }
    }

    fn block_comment(code: &mut CodeStream) {
        code.skip(2);
        while !Self::is_block_comment_end(code) {
            code.accept();
        }
        code.skip(2);
    }

    fn is_line_comment_begin(code: &CodeStream) -> bool {
        code.check("//")
    }

    fn is_block_comment_begin(code: &CodeStream) -> bool {
        code.check("/*")
    }

    fn is_line_comment_end(code: &CodeStream) -> bool {
        code.check("\n") || code.is_eof()
    }

    fn is_block_comment_end(code: &CodeStream) -> bool {
        code.check("*/")
    }
}

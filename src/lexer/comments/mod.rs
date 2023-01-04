use self::comment_type::CommentType;

use super::code_stream::CodeStream;

mod comment_type;

const COMMENTS: [CommentType; 2] = [
    CommentType {
        begin: "//",
        end: "\n",
    },
    CommentType {
        begin: "/*",
        end: "*/",
    },
];
pub struct CommentsHandler;

impl CommentsHandler {
    pub fn skip(code: &mut CodeStream) {
        Self::skip_spaces(code);

        for comment_type in COMMENTS.iter() {
            comment_type.try_skip(code);
        }

        Self::skip_spaces(code);

        if COMMENTS.into_iter().any(|c| c.is_begin(code)) {
            Self::skip(code);
        }
    }

    pub fn skip_spaces(code: &mut CodeStream) {
        while !code.is_eof() && code.current().is_ascii_whitespace() {
            code.accept();
        }
    }
}

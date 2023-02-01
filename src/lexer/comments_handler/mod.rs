use self::comment_type::CommentType;
use super::CodeStream;

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
    pub fn skip(code_stream: &mut CodeStream) {
        Self::skip_spaces(code_stream);

        for comment_type in COMMENTS.iter() {
            comment_type.try_skip(code_stream);
        }

        Self::skip_spaces(code_stream);

        if COMMENTS.into_iter().any(|c| c.is_begin(code_stream)) {
            Self::skip(code_stream);
        }
    }

    pub fn skip_spaces(code_stream: &mut CodeStream) {
        while !code_stream.is_eof() && code_stream.current().is_ascii_whitespace() {
            code_stream.accept();
        }
    }
}

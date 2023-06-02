pub enum TokenValue {
    /// keywords:
    Break,
    Continue,
    Else,
    Fn,
    If,
    Let,
    Return,
    While,

    /// ';'
    Semicolon,
    /// '='
    Assign,
    /// '('
    LParen,
    /// ')'
    RParen,
    /// '{'
    LBrace,
    /// '}'
    RBrace,
    /// ','
    Comma,

    /// '==' (equal)
    Eq,
    /// '!=' (not equal)
    Ne,
    /// '>=' (greater or equal)
    Ge,
    /// '>'  (greater than)
    Gt,
    /// '<=' (less or equal)
    Le,
    /// '<'  (less)
    Lt,

    /// '||'
    Or,
    /// '&&'
    And,
    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '*'
    Asterisk,
    /// '/'
    Slash,
    /// '!'
    Not,

    /// 'false', '0.42_e2'
    Literal(Literal),
    /// '_foo42'
    Ident(String),

    /// end of input
    EOF,
}

pub enum Literal {
    Bool(bool),
    Num(f32),
}

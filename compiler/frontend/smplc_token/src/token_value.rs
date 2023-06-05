#[derive(Clone, Copy, PartialEq)]
pub enum TokenValue<'source> {
    /// keywords:
    Break,
    Continue,
    Else,
    Fn,
    If,
    Let,
    Return,
    While,

    /// '='
    Assign,
    /// '+='
    AddAssign,
    /// '-='
    SubAssign,
    /// '*='
    MulAssign,
    /// '/='
    DivAssign,

    /// '('
    LParen,
    /// ')'
    RParen,
    /// '{'
    LBrace,
    /// '}'
    RBrace,

    /// ';'
    Semicolon,
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
    Ident(&'source str),

    /// end of input
    EOF,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Literal {
    Bool(bool),
    Num(f32),
}

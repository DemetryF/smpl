use crate::{
    error::ParseError,
    lexer::{Token, TokenValue},
};

macro_rules! operators {
    (
        $(
            $GroupName:ident {
                $(
                    $OpName:ident: $ser:expr, $token_alt:pat, $power:expr;
                )*
            }
        ),*
    ) => {
        $(
            #[derive(Debug, PartialEq, Clone)]
            pub enum $GroupName {
                $(
                    $OpName,
                )*
            }

            impl TryFrom<&Token> for $GroupName {
                type Error = ParseError;

                fn try_from(token: &Token) -> Result<Self, Self::Error> {
                    let op = match token.value {
                        $(
                            $token_alt => Self::$OpName,
                        )*

                        _ => return Err(ParseError::unexpected_token(token.clone())),
                    };

                    Ok(op)
                }
            }

            impl $GroupName {
                pub fn power(&self) -> (usize, usize) {
                    match self {
                        $(
                            Self::$OpName => $power,
                        )*
                    }
                }
            }

            impl std::fmt::Display for $GroupName {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    match self {
                        $(
                            Self::$OpName => write!(f, $ser),
                        )*
                    }
                }
            }
        )*
    };
}

operators![
    //  name            char  corresponding token         power
    BinOp {
        Assignment:     "=",  TokenValue::Assignment,     (2, 1);
        Or:             "|",  TokenValue::Or,             (3, 4);
        And:            "&",  TokenValue::And,            (5, 6);
        NotEqual:       "!=", TokenValue::NotEqual,       (7, 8);
        Equal:          "==", TokenValue::Equal,          (7, 8);
        GreaterOrEqual: ">=", TokenValue::GreaterOrEqual, (9, 10);
        Greater:        ">",  TokenValue::Greater,        (9, 10);
        LessOrEqual:    "<=", TokenValue::LessOrEqual,    (9, 10);
        Less:           "<",  TokenValue::Less,           (9, 10);
        Addition:       "+",  TokenValue::Plus,           (11, 12);
        Subtraction:    "-",  TokenValue::Minus,          (11, 12);
        Multiplication: "*",  TokenValue::Star,           (13, 14);
        Division:       "/",  TokenValue::Slash,          (13, 14);
    },

    UnOp {
        Not:            "!",  TokenValue::Not,            (0, 15);
        Neg:            "-",  TokenValue::Minus,          (0, 15);
    }
];

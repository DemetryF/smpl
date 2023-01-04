use paste::paste;

macro_rules! operators {
    [ $($Case:ident = $CaseValue:literal,)* ] => {
        #[allow(dead_code)]
        #[derive(Clone, Copy, Debug)]
        pub enum Operator {
            $($Case,)*
        }

        impl Into<&str> for Operator {
            fn into(self) -> &'static str {
                paste! {
                    match self {
                        $(
                            Self::$Case => $CaseValue,
                        )*
                    }
                }
            }
        }

        impl Operator {
            pub fn all() -> Vec<Self> {
                vec![
                    $(Self::$Case,)*
                ]
            }
        }
    };
}

#[rustfmt::skip]
operators![
    // comparison
    GreaterOrEqual = ">=",
    LessOrEqual = "<=",
    Greater = ">",
    Less = "<",
    Equal = "==",
    NotEqual = "!=",

    // assignment
    AdditionAssignment = "+=",
    SubtractionAssignment = "-=",
    MultiplicationAssignment = "*=",
    DivisionAssignment = "/=",
    Assignment = "=",

    // arithmetic
    Addition = "+",
    Subtraction = "-",

    Multiplication = "*",
    Division = "/",
    WholeDivision = "\\",
    ModuloDivision = "%",

    Exponentiation = "^",

    // logical
    Not = "!",
    And = "&",
    Or = "|",
];

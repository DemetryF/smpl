use paste::paste;

macro_rules! operators {
    [ $($Case:ident = $CaseValue:literal,)* ] => {
        #[allow(dead_code)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Operator {
            $($Case,)*
        }

        impl From<Operator> for &str {
            fn from(op: Operator) -> Self {
                paste! {
                    match op {
                        $(
                            Operator::$Case => $CaseValue,
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

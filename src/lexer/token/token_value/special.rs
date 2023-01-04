use paste::paste;

macro_rules! specials {
    [ $($Case:ident = $Value:literal,)* ] => {
        #[derive(Clone, Copy, Debug)]
        pub enum Special {
            $($Case,)*
        }

        impl TryFrom<&str> for Special {
            type Error = ();

            fn try_from(arg: &str) -> Result<Self, Self::Error> {
                paste! {
                    match arg {
                        $(
                            $Value => Ok(Self::$Case),
                        )*
                        _ => Err(())
                    }
                }
            }
        }
    };
}

#[rustfmt::skip]
specials![
    Comma = ",",
    Colon = ";",
    OpeningParen = "(",
    ClosingParen = ")",
    OpeningBrace = "{",
    ClosingBrace = "}",
];

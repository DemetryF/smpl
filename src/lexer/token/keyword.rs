use paste::paste;

macro_rules! keywords {
    [ $($Case:ident,)* ] => {
        #[derive(Clone, Copy, Debug)]
        pub enum Keyword {
            $($Case,)*
        }

        impl TryFrom<&str> for Keyword {
            type Error = ();

            fn try_from(arg: &str) -> Result<Self, Self::Error> {
                paste! {
                    match arg {
                        $(stringify!([<$Case:snake>]) => Ok(Self::$Case),)*
                        _ => Err(())
                    }
                }
            }
        }
    };
}

#[rustfmt::skip]
keywords![
    Define,
    Else, 
    Function, 
    If, 
    Return, 
    While,
];

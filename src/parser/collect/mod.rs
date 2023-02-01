use super::token_stream::TokenStream;
use crate::{
    ast::{Atom, Id},
    error::*,
};
use std::fmt::Display;

pub mod block;
pub mod expr;
pub mod statement;

pub trait Collect: Sized {
    fn collect(token_stream: &mut TokenStream) -> Result<Self>;
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{id}"),
            Self::Temp(id) => write!(f, "%{id}"),
            Self::Literal(literal) => write!(f, "{literal}"),
        }
    }
}

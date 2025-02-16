use std::fmt;
use std::hash::Hash;
use std::ops::Index;

use crate::Pos;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spanned<T>(pub T, pub Span);

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self(value, span)
    }

    pub fn span(&self) -> Span {
        self.1
    }
}

impl<T: Hash> Hash for Spanned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl<T: Eq> Eq for Spanned<T> {}

pub trait MakeSpanned: Sized {
    fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned::new(self, span)
    }
}

impl<T> MakeSpanned for T {}

// implementing of Default trait is temporary solution and in the future will be removed
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct Span {
    start: Pos,
    len: usize,
}

impl Span {
    pub fn with_len(start: Pos, len: usize) -> Self {
        Self { start, len }
    }

    pub fn with_end(start: Pos, end: usize) -> Self {
        Self::with_len(start, end - start.index())
    }

    pub fn unite(lhs: Self, rhs: Self) -> Self {
        Self::with_end(lhs.start(), rhs.end())
    }

    #[inline(always)]
    pub fn start(self) -> Pos {
        self.start
    }

    #[inline(always)]
    pub fn len(self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn end(self) -> usize {
        self.start().index() + self.len()
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, span: Span) -> &Self::Output {
        &self[span.start.index()..span.end()]
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.start())
    }
}

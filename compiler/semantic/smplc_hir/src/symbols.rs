use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

use smplc_ast as ast;
use smplc_ast::Type;

#[derive(Default)]
pub struct Symbols<'source> {
    pub functions: SymbolsTable<FunId, FunData<'source>>,
    pub variables: SymbolsTable<VarId, VarData<'source>>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct FunId(usize);

impl From<usize> for FunId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct VarId(pub usize);

impl From<usize> for VarId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunData<'source> {
    pub id: ast::Id<'source>,
    pub ret_ty: Option<Type>,
    pub args_types: Vec<Type>,
}

#[derive(Hash)]
pub struct VarData<'source> {
    pub id: ast::Id<'source>,
    pub ty: Option<Type>,
}

pub struct SymbolsTable<K: Hash + Eq + From<usize> + Copy, V> {
    data: HashMap<K, V>,
    counter: usize,
}

impl<K: Hash + Eq + From<usize> + Copy, V> SymbolsTable<K, V> {
    pub fn add(&mut self, data: V) -> K {
        let id = self.next_id();

        self.data.insert(id, data);

        id
    }

    fn next_id(&mut self) -> K {
        self.counter += 1;

        K::from(self.counter)
    }
}

impl<K: Hash + Eq + From<usize> + Copy, V> Index<K> for SymbolsTable<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        &self.data[&index]
    }
}

impl<K: Hash + Eq + From<usize> + Copy, V> IndexMut<K> for SymbolsTable<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.data.get_mut(&index).unwrap()
    }
}

impl<K: Hash + Eq + From<usize> + Copy, V> Default for SymbolsTable<K, V> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            counter: Default::default(),
        }
    }
}

use std::{cell::Cell, collections::HashMap};

use smplc_lir::{Id, Number, Type};
use smplc_thir::VarId;

#[derive(Default)]
pub struct BaseIdents<'p> {
    pub parent: Option<&'p Self>,
    pub variables: HashMap<VarId, Id>,
    pub constants: HashMap<Id, Number>,
    counter: Cell<usize>,
}

impl<'p> BaseIdents<'p> {
    pub fn with_parent(parent: &'p Self) -> Self {
        Self {
            parent: Some(parent),
            ..Default::default()
        }
    }

    pub fn get(&self, var: VarId) -> Id {
        self.variables
            .get(&var)
            .copied()
            .unwrap_or_else(|| self.parent.as_ref().unwrap().get(var))
    }

    pub fn try_get(&self, var: VarId) -> Option<Id> {
        self.variables
            .get(&var)
            .copied()
            .or_else(|| self.parent.and_then(|parent| parent.try_get(var)))
    }

    pub fn add(&mut self, var: VarId, ty: Type) -> Id {
        let id = self.next(ty);

        self.set(var, id);

        id
    }

    pub fn next(&self, ty: Type) -> Id {
        if let Some(parent) = self.parent.as_deref() {
            parent.next(ty)
        } else {
            let id = Id::new(self.counter.get(), ty);

            self.counter.set(self.counter.get() + 1);

            id
        }
    }

    pub fn set(&mut self, var: VarId, id: Id) {
        self.variables.insert(var, id);
    }
}

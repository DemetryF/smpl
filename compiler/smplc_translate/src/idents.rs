use std::{cell::Cell, collections::HashMap};

use comet_ir::{Id, Value};
use smplc_thir::VarId;

#[derive(Default)]
pub struct BaseIdents<'p> {
    pub parent: Option<&'p Self>,
    pub variables: HashMap<VarId, Id>,
    pub constants: HashMap<Id, Value>,
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

    pub fn add(&mut self, var: VarId) -> Id {
        let id = self.next();

        self.set(var, id);

        id
    }

    pub fn next(&self) -> Id {
        if let Some(parent) = self.parent.as_deref() {
            parent.next()
        } else {
            let id = Id::new(self.counter.get());

            self.counter.set(self.counter.get() + 1);

            id
        }
    }

    pub fn set(&mut self, var: VarId, id: Id) {
        self.variables.insert(var, id);
    }
}

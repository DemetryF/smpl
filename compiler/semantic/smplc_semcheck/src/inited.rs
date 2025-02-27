use std::collections::HashSet;

use smplc_hir::VarId;

// WARNING! THIS FILE CONTAINS A BRILLIANT ARCHITECTORY
// COVERED IN ENDLESS FRACTAL ABSTRACTIONS

pub trait Inited {
    fn add(&mut self, vars: HashSet<VarId>);
    fn init(&mut self, var: VarId);
    fn is_inited(&self, var: VarId) -> bool;
    fn exit(self);
}

#[derive(Default)]
pub struct GeneralInited<'parent> {
    parent: Option<&'parent mut dyn Inited>,
    inited: HashSet<VarId>,
}

impl<'parent> GeneralInited<'parent> {
    pub fn with_parent(parent: &'parent mut dyn Inited) -> Self {
        Self {
            parent: Some(parent),
            ..Default::default()
        }
    }
}

impl Inited for GeneralInited<'_> {
    fn add(&mut self, vars: HashSet<VarId>) {
        self.inited.extend(vars);
    }

    fn init(&mut self, var: VarId) {
        self.inited.insert(var);
    }

    fn is_inited(&self, var: VarId) -> bool {
        self.inited.contains(&var)
            || self
                .parent
                .as_ref()
                .is_some_and(|parent| parent.is_inited(var))
    }

    fn exit(self) {
        let Some(parent) = self.parent else {
            return;
        };

        parent.add(self.inited);
    }
}

/// requires initializing in all the nested Inited
/// useful in if statements
#[derive(Default)]
pub struct AndInited<'parent> {
    parent: Option<&'parent mut dyn Inited>,
    first: Option<HashSet<VarId>>,
}

impl<'parent> AndInited<'parent> {
    pub fn with_parent(parent: &'parent mut dyn Inited) -> Self {
        Self {
            parent: Some(parent),
            ..Default::default()
        }
    }
}

impl Inited for AndInited<'_> {
    fn add(&mut self, vars: HashSet<VarId>) {
        if let Some(first) = &mut self.first {
            first.retain(|var| vars.contains(var));
        } else {
            self.first = Some(vars)
        }
    }

    fn init(&mut self, _var: VarId) {
        unreachable!()
    }

    fn exit(self) {
        let Some(parent) = self.parent else {
            return;
        };

        let Some(first) = self.first else {
            return;
        };

        parent.add(first);
    }

    fn is_inited(&self, var: VarId) -> bool {
        // it skips the self layer

        self.parent
            .as_ref()
            .is_some_and(|parent| parent.is_inited(var))
    }
}

/// init variables only at it level and doesn't pass them below
/// useful in loops
#[derive(Default)]
pub struct NothingInited<'parent> {
    parent: Option<&'parent mut dyn Inited>,
    inited: HashSet<VarId>,
}

impl<'parent> NothingInited<'parent> {
    pub fn with_parent(parent: &'parent mut dyn Inited) -> Self {
        Self {
            parent: Some(parent),
            ..Default::default()
        }
    }
}

impl Inited for NothingInited<'_> {
    fn add(&mut self, vars: HashSet<VarId>) {
        self.inited.extend(vars);
    }

    fn init(&mut self, var: VarId) {
        self.inited.insert(var);
    }

    fn is_inited(&self, var: VarId) -> bool {
        self.inited.contains(&var)
            || self
                .parent
                .as_ref()
                .is_some_and(|parent| parent.is_inited(var))
    }

    fn exit(self) {}
}

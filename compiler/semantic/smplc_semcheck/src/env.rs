use std::{collections::HashMap, rc::Rc};

use smplc_hir::{FunData, FunRef, VarData, VarRef};

use crate::error::{SemError, SemResult};

#[derive(Default)]
pub struct Env<'source> {
    pub variables: Variables<'source>,
    pub functions: Functions<'source>,
}

#[derive(Default)]
pub struct Variables<'source> {
    data: Vec<Scope<'source, VarRef>>,
}

#[derive(Default)]
pub struct Functions<'source> {
    data: Scope<'source, FunRef>,
}

pub struct Scope<'source, V: Clone> {
    data: HashMap<&'source str, V>,
}

impl<'source> Variables<'source> {
    pub fn fork(&mut self) {
        self.data.push(Scope::default());
    }

    pub fn exit(&mut self) {
        self.data.pop();
    }

    fn last(&self) -> &Scope<'source, VarRef> {
        self.data.last().unwrap()
    }

    fn last_mut(&mut self) -> &mut Scope<'source, VarRef> {
        self.data.last_mut().unwrap()
    }

    pub fn get(&self, id: smplc_ast::Id<'source>) -> SemResult<'source, VarRef> {
        for scope in self.data.iter().rev() {
            if let Some(var_ref) = scope.get(id.id) {
                return Ok(var_ref);
            }
        }

        Err(SemError::non_existent_variable(id))
    }

    pub fn add_variable(&mut self, id: smplc_ast::Id<'source>) -> SemResult<'source, VarRef> {
        if let Some(var_ref) = self.last().get(id.id) {
            Err(SemError::redeclaring_variable(id, var_ref.declared_at))
        } else {
            let smplc_ast::Id {
                id,
                pos: declared_at,
            } = id;

            let var_ref = Rc::new(VarData { declared_at });

            self.last_mut().add(id, Rc::clone(&var_ref));

            Ok(var_ref)
        }
    }

    pub fn add_argument(&mut self, id: smplc_ast::Id<'source>) -> SemResult<'source, VarRef> {
        if self.last().has(id.id) {
            Err(SemError::duplicate_args_names(id))
        } else {
            let smplc_ast::Id {
                id,
                pos: declared_at,
            } = id;

            let var_ref = Rc::new(VarData { declared_at });

            self.last_mut().add(id, Rc::clone(&var_ref));

            Ok(var_ref)
        }
    }
}

impl<'source> Functions<'source> {
    pub fn get(&self, id: smplc_ast::Id<'source>) -> SemResult<'source, FunRef> {
        self.data
            .get(id.id)
            .ok_or_else(|| SemError::non_existent_function(id))
    }

    pub fn add(
        &mut self,
        id: smplc_ast::Id<'source>,
        args_count: usize,
    ) -> SemResult<'source, FunRef> {
        if let Some(fun_ref) = self.data.get(id.id) {
            Err(SemError::redeclaring_function(id, fun_ref.declared_at))
        } else {
            let smplc_ast::Id { id, pos } = id;

            let fun_ref = Rc::new(FunData {
                declared_at: pos,
                id: smplc_ir::FunctionId(id.into()),
                args_count,
            });

            self.data.add(id, Rc::clone(&fun_ref));

            Ok(fun_ref)
        }
    }
}

impl<'source, V: Clone> Scope<'source, V> {
    pub fn get(&self, id: &'source str) -> Option<V> {
        self.data.get(id).cloned()
    }

    pub fn add(&mut self, id: &'source str, value: V) {
        self.data.insert(id, value);
    }

    pub fn has(&self, id: &'source str) -> bool {
        self.data.contains_key(id)
    }
}

impl<V: Clone> Default for Scope<'_, V> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

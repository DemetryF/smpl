use std::collections::HashMap;

use smplc_ast as ast;
use smplc_hir::{FunData, FunId, SymbolsTable, Type, VarData, VarId};

use crate::{
    error::{SemError, SemResult},
    semcheck::RawType,
};

#[derive(Default)]
pub struct Env<'source> {
    pub variables: Variables<'source>,
    pub functions: Functions<'source>,

    pub current_fn: Option<FunId>,
}

#[derive(Default)]
pub struct Variables<'source> {
    data: Vec<Scope<'source, VarId>>,

    pub symbols: SymbolsTable<VarId, VarData<'source>>,
}

impl<'source> Variables<'source> {
    pub fn fork(&mut self) {
        self.data.push(Scope::default());
    }

    pub fn exit(&mut self) {
        self.data.pop();
    }

    fn last(&self) -> &Scope<'source, VarId> {
        self.data.last().unwrap()
    }

    fn last_mut(&mut self) -> &mut Scope<'source, VarId> {
        self.data.last_mut().unwrap()
    }

    pub fn get(&self, id: ast::Id<'source>) -> SemResult<'source, VarId> {
        for scope in self.data.iter().rev() {
            if let Some(var_ref) = scope.get(id.0) {
                return Ok(var_ref);
            }
        }

        Err(SemError::non_existent_variable(id))
    }

    pub fn add_variable(
        &mut self,
        id: ast::Id<'source>,
        ty: Option<Type>,
    ) -> SemResult<'source, VarId> {
        if let Some(symbol_id) = self.last().get(id.0) {
            let first_declaration = self.symbols[symbol_id].id.span();

            Err(SemError::redeclaring_variable(id, first_declaration))
        } else {
            let var_data = VarData { id, ty };
            let var_id = self.symbols.add(var_data);

            self.last_mut().add(id.0, var_id);

            Ok(var_id)
        }
    }

    pub fn add_argument(&mut self, arg: ast::FunctionArg<'source>) -> SemResult<'source, VarId> {
        if self.last().has(arg.id.0) {
            Err(SemError::duplicate_args_names(arg.id))
        } else {
            let var_data = VarData {
                id: arg.id,
                ty: Some(RawType(arg.ty).checked()?),
            };

            let var_id = self.symbols.add(var_data);

            self.last_mut().add(arg.id.0, var_id);

            Ok(var_id)
        }
    }
}

#[derive(Default)]
pub struct Functions<'source> {
    data: Scope<'source, FunId>,

    pub symbols: SymbolsTable<FunId, FunData<'source>>,
}

impl<'source> Functions<'source> {
    pub fn get(&self, id: ast::Id<'source>) -> SemResult<'source, FunId> {
        self.data
            .get(id.0)
            .ok_or_else(|| SemError::non_existent_function(id))
    }

    pub fn add(
        &mut self,
        id: ast::Id<'source>,
        args_types: Vec<Type>,
        ret_ty: Option<Type>,
    ) -> SemResult<'source, FunId> {
        if let Some(fun_id) = self.data.get(id.0) {
            let first_declaration = self.symbols[fun_id].id.span();

            Err(SemError::redeclaring_function(id, first_declaration))
        } else {
            let fun_data = FunData {
                id,
                args_types,
                ret_ty,
            };

            let fun_id = self.symbols.add(fun_data);

            self.data.add(id.0, fun_id);

            Ok(fun_id)
        }
    }
}

pub struct Scope<'source, V: Clone> {
    data: HashMap<&'source str, V>,
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

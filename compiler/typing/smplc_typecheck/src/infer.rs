mod expr;
mod statement;

use std::collections::HashMap;
use std::ops::Index;

use smplc_hir as hir;
use smplc_hir::Type;
use smplc_thir::{FunId, Symbols, VarData, VarId};

pub use self::expr::infer_expr;

use crate::error::{TypeError, TypeErrorType, TypeResult};

pub trait TypeInfer<'source> {
    fn infer(
        &self,
        inferrer: &mut TypeInferrer,
        symbols: &hir::Symbols<'source>,
    ) -> TypeResult<'source, ()>;
}

#[derive(Default)]
pub struct TypeInferrer {
    pub vars: HashMap<VarId, SetId>,
    pub sets: HashMap<SetId, TypeVar>,

    sets_counter: usize,

    pub current_fn: Option<FunId>,
}

impl TypeInferrer {
    pub fn set_var_ty(&mut self, var: VarId, ty: TypeVar) -> Result<SetId, (TypeVar, TypeVar)> {
        match self.vars.get(&var) {
            Some(&set) => {
                let ty = TypeVar::max(self.sets[&set], ty)?;

                self.sets.insert(set, ty);

                Ok(set)
            }

            None => {
                let set = self.next_set_id();

                self.vars.insert(var, set);
                self.sets.insert(set, ty);

                Ok(set)
            }
        }
    }

    pub fn set_set_ty(&mut self, set: SetId, ty: TypeVar) -> Result<TypeVar, (TypeVar, TypeVar)> {
        let new_ty = TypeVar::max(self.sets[&set], ty)?;

        self.sets.insert(set, new_ty);

        Ok(new_ty)
    }

    pub fn unite(&mut self, a: SetId, b: SetId) -> Result<SetId, (TypeVar, TypeVar)> {
        if a == b {
            return Ok(a);
        }

        let ty = self.sets.remove(&b).unwrap();

        self.sets.insert(a, TypeVar::max(self.sets[&a], ty)?);

        self.vars
            .iter_mut()
            .filter(|(_, &mut set)| set == b)
            .for_each(|(_, set)| *set = a);

        Ok(a)
    }

    fn next_set_id(&mut self) -> SetId {
        self.sets_counter += 1;

        SetId(self.sets_counter)
    }

    pub fn infer(self, symbols: hir::Symbols) -> Result<Symbols, Vec<TypeError>> {
        let no_all_infered = self
            .sets
            .iter()
            .any(|(_, ty)| !matches!(ty, TypeVar::Type(_)));

        if no_all_infered {
            Err(self
                .vars
                .into_iter()
                .map(|(var, set)| (var, self.sets[&set]))
                .filter(|(_, ty)| !matches!(ty, TypeVar::Type(_)))
                .map(|(var, type_var)| {
                    let var = symbols.variables[var].id;

                    TypeError {
                        kind: TypeErrorType::CouldNotInfer {
                            var_id: var.0,
                            type_var,
                        },
                        span: var.span(),
                    }
                })
                .collect())
        } else {
            let variables = symbols
                .variables
                .into_iter()
                .map(|(id, data)| {
                    let var_data = VarData {
                        id: data.id,
                        ty: self.ty(id),
                    };

                    (id, var_data)
                })
                .collect();

            Ok(Symbols {
                functions: symbols.functions,
                variables,
            })
        }
    }

    fn ty(&self, var: VarId) -> Type {
        let TypeVar::Type(ty) = self.sets[&self.vars[&var]] else {
            unreachable!()
        };

        ty
    }
}

pub struct TypesInfo {
    vars: HashMap<VarId, SetId>,
    sets: HashMap<SetId, TypeVar>,
}

impl Index<VarId> for TypesInfo {
    type Output = Type;

    fn index(&self, index: VarId) -> &Self::Output {
        let TypeVar::Type(ty) = &self.sets[&self.vars[&index]] else {
            unreachable!()
        };

        ty
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeVar {
    Type(Type),
    Number,
    Unknown,
    None,
}

impl TypeVar {
    pub fn max(a: Self, b: Self) -> Result<Self, (Self, Self)> {
        match (a, b) {
            (a, b) if a == b => Ok(a),

            (Self::Unknown, res)
            | (res, Self::Unknown)
            | (Self::Number, res @ Self::Type(Type::Int | Type::Real))
            | (res @ Self::Type(Type::Int | Type::Real), Self::Number) => Ok(res),

            _ => Err((a, b)),
        }
    }
}

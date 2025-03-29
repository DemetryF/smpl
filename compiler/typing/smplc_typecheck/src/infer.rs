mod expr;
mod statement;

use std::{collections::HashMap, ops::Index};

use smplc_ast::Spanned;
use smplc_hir as hir;
use smplc_hir::Type;
use smplc_thir::{FunId, Symbols, VarData, VarId};

use crate::{
    error::{TypeError, TypeErrorKind, TypeResult},
    type_var::TypeVar,
};

use self::expr::InferenceResult;

pub use self::expr::infer_expr;

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

    relations: Vec<Relation>,
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

    pub fn assume_inference<'source>(
        &mut self,
        inference: InferenceResult,
        ty: TypeVar,
    ) -> TypeResult<'source, TypeVar> {
        if let Some(set) = inference.set {
            self.set_set_ty(set, ty)
        } else {
            TypeVar::max(inference.ty, ty)
        }
        .map_err(|(got, required)| TypeError::mismatched_types(required, got, inference.span))
    }

    pub fn try_unite(
        &mut self,
        a: Option<SetId>,
        b: Option<SetId>,
    ) -> Result<Option<SetId>, (TypeVar, TypeVar)> {
        if let (Some(a), Some(b)) = (a, b) {
            self.unite(a, b).map(Some)
        } else {
            Ok(a.or(b))
        }
    }

    pub fn connect(&mut self, relation: Relation) {
        self.relations.push(relation);
    }

    pub fn unite(&mut self, a: SetId, b: SetId) -> Result<SetId, (TypeVar, TypeVar)> {
        if a == b {
            return Ok(a);
        }

        let new_ty = TypeVar::max(self.sets[&a], self.sets[&b])?;

        self.set_set_ty(a, new_ty)?;

        // self.sets.remove(&b).unwrap();

        for (_, set) in &mut self.vars {
            if set == &b {
                *set = a
            }
        }

        Ok(a)
    }

    fn next_set_id(&mut self) -> SetId {
        self.sets_counter += 1;

        SetId(self.sets_counter)
    }

    pub fn solve_relations<'source>(&mut self) -> TypeResult<'source, ()> {
        for relation in std::mem::take(&mut self.relations) {
            match relation {
                Relation::Mul(lhs, rhs) => {
                    let lhs_set = lhs.0;
                    let lhs_ty = self.sets[&lhs_set];

                    let rhs_set = rhs.0;
                    let rhs_ty = self.sets[&rhs_set];

                    if lhs_ty.is_vec() {
                        self.set_set_ty(rhs_set, Type::Real.into())
                            .map(|_| ())
                            .map_err(|tys| (tys, rhs.span()))
                    } else if rhs_ty.is_vec() {
                        self.set_set_ty(lhs_set, Type::Real.into())
                            .map(|_| ())
                            .map_err(|tys| (tys, lhs.span()))
                    } else if lhs_ty.is_number() && rhs_ty.is_number() {
                        self.unite(lhs_set, rhs_set)
                            .map(|_| ())
                            .map_err(|tys| (tys, rhs.span()))
                    } else {
                        Err(((lhs_ty, rhs_ty), rhs.span()))
                    }
                    .map_err(|((got, required), span)| {
                        TypeError::mismatched_types(required, got, span)
                    })?;
                }
                Relation::Div(lhs, rhs) => {
                    let lhs_set = lhs.0;
                    let lhs_ty = self.sets[&lhs_set];

                    let rhs_set = rhs.0;
                    let rhs_ty = self.sets[&rhs_set];

                    if lhs_ty.is_vec() {
                        self.set_set_ty(rhs_set, Type::Real.into())
                            .map(|_| ())
                            .map_err(|tys| (tys, rhs.span()))
                    } else if lhs_ty.is_number() && rhs_ty.is_number() {
                        self.unite(lhs_set, rhs_set)
                            .map(|_| ())
                            .map_err(|tys| (tys, rhs.span()))
                    } else {
                        Err(((lhs_ty, rhs_ty), rhs.span()))
                    }
                    .map_err(|((got, required), span)| {
                        TypeError::mismatched_types(required, got, span)
                    })?;
                }
            }
        }

        Ok(())
    }

    pub fn infer(mut self, symbols: hir::Symbols) -> Result<Symbols, Vec<TypeError>> {
        self.solve_relations().map_err(|err| vec![err])?;

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
                        kind: TypeErrorKind::CouldNotInfer {
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

pub enum Relation {
    Mul(Spanned<SetId>, Spanned<SetId>),
    Div(Spanned<SetId>, Spanned<SetId>),
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

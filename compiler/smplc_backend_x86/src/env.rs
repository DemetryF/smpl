use std::cmp::Ordering;
use std::collections::HashMap;

use smplc_lir as ir;
use smplc_lir::{Label, Phi, Type};

pub struct Env<'a> {
    pub functions: &'a HashMap<ir::FunId, String>,
    pub labels: &'a HashMap<Label, String>,
    pub phis: &'a Vec<Phi>,
    constants: &'a HashMap<ir::Id, String>,
    id_types: &'a HashMap<ir::Id, Type>,

    addresses: HashMap<ir::Id, isize>,
    vars_count: usize,
}

impl<'a> Env<'a> {
    pub fn new(
        constants: &'a HashMap<ir::Id, String>,
        labels: &'a HashMap<Label, String>,
        phis: &'a Vec<Phi>,
        functions: &'a HashMap<ir::FunId, String>,
        id_types: &'a HashMap<ir::Id, Type>,
    ) -> Self {
        Self {
            functions,
            labels,
            phis,
            constants,
            id_types,
            addresses: Default::default(),
            vars_count: Default::default(),
        }
    }

    pub fn get(&self, id: ir::Id) -> String {
        if let Some(address) = self.constants.get(&id) {
            return address.clone();
        }

        let address = self.addresses[&id] * 8;

        let ordering = address.cmp(&0);
        let address = address.abs();

        match ordering {
            Ordering::Less => format!("DWORD [rbp+{address}]"),
            Ordering::Greater => format!("DWORD [rbp-{address}]"),

            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, id: ir::Id, address: isize) {
        self.addresses.insert(id, address);
    }

    pub fn get_or_add(&mut self, id: ir::Id) -> String {
        if !self.addresses.contains_key(&id) {
            self.vars_count += 1;
            self.set(id, self.vars_count as isize);
        }

        self.get(id)
    }

    pub fn stack_size(&self) -> usize {
        self.vars_count * 8
    }

    pub fn ty(&self, id: ir::Id) -> Type {
        self.id_types[&id]
    }
}

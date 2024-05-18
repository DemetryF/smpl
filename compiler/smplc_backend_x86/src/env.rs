use std::cmp::Ordering;
use std::collections::HashMap;

use smplc_lir as ir;

pub struct Env<'a> {
    constants: &'a HashMap<ir::Id, String>,

    addresses: HashMap<ir::Id, isize>,
    vars_count: usize,
}

impl<'a> Env<'a> {
    pub fn new(constants: &'a HashMap<ir::Id, String>) -> Self {
        Self {
            constants,

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
}

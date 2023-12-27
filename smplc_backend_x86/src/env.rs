use std::cmp::Ordering;
use std::collections::HashMap;

use smplc_ir::Id;

#[derive(Default)]
pub struct Env {
    addresses: HashMap<Id, isize>,
    pub variables_count: usize,
}

impl Env {
    pub fn get(&self, id: Id) -> String {
        let address = self.addresses[&id] * 8;

        let ordering = address.cmp(&0);
        let address = address.abs();

        match ordering {
            Ordering::Less => format!("DWORD [rbp+{address}]"),
            Ordering::Greater => format!("DWORD [rbp-{address}]"),

            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, id: Id, address: isize) {
        self.addresses.insert(id, address);
    }

    pub fn add(&mut self, id: Id) -> String {
        if !self.addresses.contains_key(&id) {
            self.variables_count += 1;
            self.set(id.clone(), self.variables_count as isize);
        }

        self.get(id)
    }

    pub fn size(&self) -> usize {
        self.variables_count * 8
    }
}

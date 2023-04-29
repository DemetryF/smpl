use std::{cmp::Ordering, collections::HashMap};

#[derive(Default)]
pub struct Env {
    addresses: HashMap<String, isize>,
    pub variables_count: usize,
}

impl Env {
    pub fn get(&self, id: &String) -> String {
        let address = self.addresses[id] * 8;

        let ordering = address.cmp(&0);
        let address = address.abs();

        match ordering {
            Ordering::Less => format!("DWORD [rbp+{address}]"),
            Ordering::Greater => format!("DWORD [rbp-{address}]"),

            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, id: String, address: isize) {
        self.addresses.insert(id, address);
    }

    pub fn add(&mut self, id: &String) -> String {
        if !self.addresses.contains_key(id) {
            self.variables_count += 1;
            self.set(id.clone(), self.variables_count as isize);
        }

        self.get(id)
    }

    pub fn size(&self) -> usize {
        self.variables_count * 8
    }
}

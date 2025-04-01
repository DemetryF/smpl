use std::{cmp::Ordering, collections::HashMap, fmt};

use smplc_lir as ir;
use smplc_lir::{Label, Phi};

pub struct Env<'a> {
    pub functions: &'a HashMap<ir::FunId, String>,
    pub labels: &'a HashMap<Label, String>,
    pub phis: &'a Vec<Phi>,
    constants: &'a HashMap<ir::Id, Operand>,

    addresses: HashMap<ir::Id, isize>,
    vars_count: usize,
}

impl<'a> Env<'a> {
    pub fn new(
        constants: &'a HashMap<ir::Id, Operand>,
        labels: &'a HashMap<Label, String>,
        phis: &'a Vec<Phi>,
        functions: &'a HashMap<ir::FunId, String>,
    ) -> Self {
        Self {
            functions,
            labels,
            phis,
            constants,
            addresses: Default::default(),
            vars_count: Default::default(),
        }
    }

    pub fn get(&self, id: ir::Id) -> Operand {
        if let Some(&address) = self.constants.get(&id) {
            return address;
        }

        Operand::Address(Address::Stack(self.addresses[&id]))
    }

    pub fn has(&self, id: ir::Id) -> bool {
        self.addresses.contains_key(&id)
    }

    pub fn addr(&self, id: ir::Id) -> isize {
        self.addresses[&id]
    }

    pub fn set(&mut self, id: ir::Id, address: isize) {
        self.addresses.insert(id, address);
    }

    pub fn get_or_add(&mut self, id: ir::Id) -> Operand {
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Address(Address),
    Number(i32),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Address(address) => write!(f, "{address}"),
            Operand::Number(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Address {
    Stack(isize),
    Const(usize),
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Address::Stack(address) => {
                let ordering = address.cmp(&0);
                let address = 8 * address.abs();

                match ordering {
                    Ordering::Less => write!(f, "[rbp+{address}]"),
                    Ordering::Greater => write!(f, "[rbp-{address}]"),

                    _ => unreachable!(),
                }
            }
            &Address::Const(n) => {
                write!(f, "[LC{n}]")
            }
        }
    }
}

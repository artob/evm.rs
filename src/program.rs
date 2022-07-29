// This is free and unencumbered software released into the public domain.

use std::{collections::BTreeSet, iter, slice};

use crate::opcode::Opcode;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Program(pub Vec<Opcode>);

impl Program {
    pub fn opcode_set(&self) -> BTreeSet<Opcode> {
        let mut result = BTreeSet::new();
        for op in &self.0 {
            if !result.contains(op) {
                result.insert(op.zeroed());
            }
        }
        result
    }
}

impl<'a> iter::IntoIterator for &'a Program {
    type Item = Opcode;
    type IntoIter = iter::Cloned<slice::Iter<'a, Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().cloned()
    }
}

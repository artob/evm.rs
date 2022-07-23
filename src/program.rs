// This is free and unencumbered software released into the public domain.

use std::collections::BTreeSet;

use crate::opcode::Opcode;

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

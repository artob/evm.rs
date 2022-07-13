// This is free and unencumbered software released into the public domain.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DecodeError {
    InvalidBytecode,
    InvalidOpcode(u8),
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DecodeError::*;
        match *self {
            InvalidBytecode => write!(f, "invalid EVM bytecode"),
            InvalidOpcode(opcode) => write!(f, "invalid EVM opcode 0x{:02X}", opcode),
        }
    }
}

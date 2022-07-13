// This is free and unencumbered software released into the public domain.

use ethnum::u256;

use crate::{
    error::DecodeError,
    opcode::{Opcode, Program},
};

pub fn decode_program(input: &[u8]) -> Result<Program, DecodeError> {
    let mut result = Vec::new();
    let mut input_pos: usize = 0;
    while input_pos < input.len() {
        match decode_opcode(&input[input_pos..]) {
            Err(err) => return Err(err),
            Ok(op) => {
                input_pos += op.size();
                result.push(op);
            }
        }
    }
    Ok(result)
}

pub fn decode_opcode(input: &[u8]) -> Result<Opcode, DecodeError> {
    use Opcode::*;
    let opcode = input[0];
    let invalid = Err(DecodeError::InvalidOpcode(opcode));
    let result = match opcode {
        0x00 => STOP,
        0x01 => ADD,
        0x02 => MUL,
        0x03 => SUB,
        0x04 => DIV,
        0x05 => SDIV,
        0x06 => MOD,
        0x07 => SMOD,
        0x08 => ADDMOD,
        0x09 => MULMOD,
        0x0A => EXP,
        0x0B => SIGNEXTEND,
        0x0C..=0x0F => return invalid,
        0x10 => LT,
        0x11 => GT,
        0x12 => SLT,
        0x13 => SGT,
        0x14 => EQ,
        0x15 => ISZERO,
        0x16 => AND,
        0x17 => OR,
        0x18 => XOR,
        0x19 => NOT,
        0x1A => BYTE,
        0x1B => SHL,
        0x1C => SHR,
        0x1D => SAR,
        0x1E..=0x1F => return invalid,
        0x20 => SHA3,
        0x21..=0x2F => return invalid,
        0x30 => ADDRESS,
        0x31 => BALANCE,
        0x32 => ORIGIN,
        0x33 => CALLER,
        0x34 => CALLVALUE,
        0x35 => CALLDATALOAD,
        0x36 => CALLDATASIZE,
        0x37 => CALLDATACOPY,
        0x38 => CODESIZE,
        0x39 => CODECOPY,
        0x3A => GASPRICE,
        0x3B => EXTCODESIZE,
        0x3C => EXTCODECOPY,
        0x3D => RETURNDATASIZE,
        0x3E => RETURNDATACOPY,
        0x3F => EXTCODEHASH,
        0x40 => BLOCKHASH,
        0x41 => COINBASE,
        0x42 => TIMESTAMP,
        0x43 => NUMBER,
        0x44 => DIFFICULTY,
        0x45 => GASLIMIT,
        0x46 => CHAINID,
        0x47 => SELFBALANCE,
        0x48 => BASEFEE,
        0x49..=0x4F => return invalid,
        0x50 => POP,
        0x51 => MLOAD,
        0x52 => MSTORE,
        0x53 => MSTORE8,
        0x54 => SLOAD,
        0x55 => SSTORE,
        0x56 => JUMP,
        0x57 => JUMPI,
        0x58 => PC,
        0x59 => MSIZE,
        0x5A => GAS,
        0x5B => JUMPDEST,
        0x5C..=0x5F => return invalid,
        0x60 => PUSH1(input[1]),
        0x61..=0x7F => {
            let n = opcode - 0x60 + 1;
            let mut buffer: [u8; 32] = [0; 32];
            buffer[(32 - (n as usize))..32].copy_from_slice(&input[1..=n.into()]);
            PUSHn(
                n,
                u256::from_be_bytes(buffer.try_into().unwrap()),
                buffer[(32 - (n as usize))..32].to_vec(),
            )
        }
        0x80..=0x8F => DUP(opcode - 0x80 + 1),
        0x90..=0x9F => SWAP(opcode - 0x90 + 1),
        0xA0..=0xA4 => LOG(opcode - 0xA0),
        0xA5..=0xEF => return invalid,
        0xF0 => CREATE,
        0xF1 => CALL,
        0xF2 => CALLCODE,
        0xF3 => RETURN,
        0xF4 => DELEGATECALL,
        0xF5 => CREATE2,
        0xF6..=0xF9 => return invalid,
        0xFA => STATICCALL,
        0xFB..=0xFC => return invalid,
        0xFD => REVERT,
        0xFE => INVALID,
        0xFF => SELFDESTRUCT,
        //opcode => return invalid,
    };
    Ok(result)
}

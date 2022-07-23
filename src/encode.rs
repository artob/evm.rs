// This is free and unencumbered software released into the public domain.

use crate::{opcode::Opcode, program::Program};

pub fn encode_program(program: Program) -> Vec<u8> {
    encode_opcodes(&program.0)
}

pub fn encode_opcodes(opcodes: &Vec<Opcode>) -> Vec<u8> {
    let mut bytecode = vec![];
    for opcode in opcodes.iter() {
        bytecode.push(encode_opcode(opcode));
        let mut operands = encode_operands(opcode);
        bytecode.append(&mut operands);
    }
    bytecode
}

pub fn encode_opcode(opcode: &Opcode) -> u8 {
    use Opcode::*;
    match opcode {
        STOP => 0x00,
        ADD => 0x01,
        MUL => 0x02,
        SUB => 0x03,
        DIV => 0x04,
        SDIV => 0x05,
        MOD => 0x06,
        SMOD => 0x07,
        ADDMOD => 0x08,
        MULMOD => 0x09,
        EXP => 0x0A,
        SIGNEXTEND => 0x0B,
        LT => 0x10,
        GT => 0x11,
        SLT => 0x12,
        SGT => 0x13,
        EQ => 0x14,
        ISZERO => 0x15,
        AND => 0x16,
        OR => 0x17,
        XOR => 0x18,
        NOT => 0x19,
        BYTE => 0x1A,
        SHL => 0x1B,
        SHR => 0x1C,
        SAR => 0x1D,
        SHA3 => 0x20,
        ADDRESS => 0x30,
        BALANCE => 0x31,
        ORIGIN => 0x32,
        CALLER => 0x33,
        CALLVALUE => 0x34,
        CALLDATALOAD => 0x35,
        CALLDATASIZE => 0x36,
        CALLDATACOPY => 0x37,
        CODESIZE => 0x38,
        CODECOPY => 0x39,
        GASPRICE => 0x3A,
        EXTCODESIZE => 0x3B,
        EXTCODECOPY => 0x3C,
        RETURNDATASIZE => 0x3D,
        RETURNDATACOPY => 0x3E,
        EXTCODEHASH => 0x3F,
        BLOCKHASH => 0x40,
        COINBASE => 0x41,
        TIMESTAMP => 0x42,
        NUMBER => 0x43,
        DIFFICULTY => 0x44,
        GASLIMIT => 0x45,
        CHAINID => 0x46,
        SELFBALANCE => 0x47,
        BASEFEE => 0x48,
        POP => 0x50,
        MLOAD => 0x51,
        MSTORE => 0x52,
        MSTORE8 => 0x53,
        SLOAD => 0x54,
        SSTORE => 0x55,
        JUMP => 0x56,
        JUMPI => 0x57,
        PC => 0x58,
        MSIZE => 0x59,
        GAS => 0x5A,
        JUMPDEST => 0x5B,
        PUSH1(_) => 0x60,
        PUSHn(n, _, _) => 0x60 + n - 1,
        DUP(n) => 0x80 + n - 1,
        SWAP(n) => 0x90 + n - 1,
        LOG(n) => 0xA0 + n,
        CREATE => 0xF0,
        CALL => 0xF1,
        CALLCODE => 0xF2,
        RETURN => 0xF3,
        DELEGATECALL => 0xF4,
        CREATE2 => 0xF5,
        STATICCALL => 0xFA,
        REVERT => 0xFD,
        INVALID => 0xFE,
        SELFDESTRUCT => 0xFF,
    }
}

pub fn encode_operands(opcode: &Opcode) -> Vec<u8> {
    use Opcode::*;
    match opcode {
        PUSH1(b) => vec![*b],
        PUSHn(_, _, bs) => bs.clone(),
        _ => vec![],
    }
}

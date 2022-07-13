// This is free and unencumbered software released into the public domain.

use ethnum::u256;
use std::fmt;

pub type Program = Vec<Opcode>;

#[derive(Clone, Debug)]
pub enum Opcode {
    STOP,                     // 0x00
    ADD,                      // 0x01
    MUL,                      // 0x02
    SUB,                      // 0x03
    DIV,                      // 0x04
    SDIV,                     // 0x05
    MOD,                      // 0x06
    SMOD,                     // 0x07
    ADDMOD,                   // 0x08
    MULMOD,                   // 0x09
    EXP,                      // 0x0A
    SIGNEXTEND,               // 0x0B
    LT,                       // 0x10
    GT,                       // 0x11
    SLT,                      // 0x12
    SGT,                      // 0x13
    EQ,                       // 0x14
    ISZERO,                   // 0x15
    AND,                      // 0x16
    OR,                       // 0x17
    XOR,                      // 0x18
    NOT,                      // 0x19
    BYTE,                     // 0x1A
    SHL,                      // 0x1B (EIP-145)
    SHR,                      // 0x1C (EIP-145)
    SAR,                      // 0x1D (EIP-145)
    SHA3,                     // 0x20
    ADDRESS,                  // 0x30
    BALANCE,                  // 0x31
    ORIGIN,                   // 0x32
    CALLER,                   // 0x33
    CALLVALUE,                // 0x34
    CALLDATALOAD,             // 0x35
    CALLDATASIZE,             // 0x36
    CALLDATACOPY,             // 0x37
    CODESIZE,                 // 0x38
    CODECOPY,                 // 0x39
    GASPRICE,                 // 0x3A
    EXTCODESIZE,              // 0x3B
    EXTCODECOPY,              // 0x3C
    RETURNDATASIZE,           // 0x3D (EIP-211)
    RETURNDATACOPY,           // 0x3E (EIP-211)
    EXTCODEHASH,              // 0x3F (EIP-1052)
    BLOCKHASH,                // 0x40
    COINBASE,                 // 0x41
    TIMESTAMP,                // 0x42
    NUMBER,                   // 0x43
    DIFFICULTY,               // 0x44
    GASLIMIT,                 // 0x45
    CHAINID,                  // 0x46 (EIP-1344)
    SELFBALANCE,              // 0x47 (EIP-1884)
    BASEFEE,                  // 0x48
    POP,                      // 0x50
    MLOAD,                    // 0x51
    MSTORE,                   // 0x52
    MSTORE8,                  // 0x53
    SLOAD,                    // 0x54
    SSTORE,                   // 0x55
    JUMP,                     // 0x56
    JUMPI,                    // 0x57
    PC,                       // 0x58
    MSIZE,                    // 0x59
    GAS,                      // 0x5A
    JUMPDEST,                 // 0x5B
    PUSH1(u8),                // 0x60
    PUSHn(u8, u256, Vec<u8>), // 0x61..=0x7F
    DUP(u8),                  // 0x80..=0x8F
    SWAP(u8),                 // 0x90..=0x9F
    LOG(u8),                  // 0xA0..=0xA4
    CREATE,                   // 0xF0
    CALL,                     // 0xF1
    CALLCODE,                 // 0xF2
    RETURN,                   // 0xF3
    DELEGATECALL,             // 0xF4 (EIP-7)
    CREATE2,                  // 0xF5 (EIP-1014)
    STATICCALL,               // 0xFA
    REVERT,                   // 0xFD (EIP-140)
    INVALID,                  // 0xFE (EIP-141)
    SELFDESTRUCT,             // 0xFF (EIP-6)
}

impl Opcode {
    pub fn size(&self) -> usize {
        use Opcode::*;
        match self {
            PUSH1(_) => 1 + 1,
            PUSHn(n, _, _) => 1 + *n as usize,
            _ => 1,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Opcode::*;
        match self {
            PUSH1(b) => write!(f, "PUSH1 0x{:02x}", b),
            PUSHn(n, _, bs) => write!(f, "PUSH{} 0x{}", n, hex::encode(bs)),
            DUP(n) => write!(f, "DUP{}", n),
            SWAP(n) => write!(f, "SWAP{}", n),
            LOG(n) => write!(f, "LOG{}", n),
            _ => {
                let s = match *self {
                    STOP => "STOP",
                    ADD => "ADD",
                    MUL => "MUL",
                    SUB => "SUB",
                    DIV => "DIV",
                    SDIV => "SDIV",
                    MOD => "MOD",
                    SMOD => "SMOD",
                    ADDMOD => "ADDMOD",
                    MULMOD => "MULMOD",
                    EXP => "EXP",
                    SIGNEXTEND => "SIGNEXTEND",
                    LT => "LT",
                    GT => "GT",
                    SLT => "SLT",
                    SGT => "SGT",
                    EQ => "EQ",
                    ISZERO => "ISZERO",
                    AND => "AND",
                    OR => "OR",
                    XOR => "XOR",
                    NOT => "NOT",
                    BYTE => "BYTE",
                    SHL => "SHL",
                    SHR => "SHR",
                    SAR => "SAR",
                    SHA3 => "SHA3",
                    ADDRESS => "ADDRESS",
                    BALANCE => "BALANCE",
                    ORIGIN => "ORIGIN",
                    CALLER => "CALLER",
                    CALLVALUE => "CALLVALUE",
                    CALLDATALOAD => "CALLDATALOAD",
                    CALLDATASIZE => "CALLDATASIZE",
                    CALLDATACOPY => "CALLDATACOPY",
                    CODESIZE => "CODESIZE",
                    CODECOPY => "CODECOPY",
                    GASPRICE => "GASPRICE",
                    EXTCODESIZE => "EXTCODESIZE",
                    EXTCODECOPY => "EXTCODECOPY",
                    RETURNDATASIZE => "RETURNDATASIZE",
                    RETURNDATACOPY => "RETURNDATACOPY",
                    EXTCODEHASH => "EXTCODEHASH",
                    BLOCKHASH => "BLOCKHASH",
                    COINBASE => "COINBASE",
                    TIMESTAMP => "TIMESTAMP",
                    NUMBER => "NUMBER",
                    DIFFICULTY => "DIFFICULTY",
                    GASLIMIT => "GASLIMIT",
                    CHAINID => "CHAINID",
                    SELFBALANCE => "SELFBALANCE",
                    BASEFEE => "BASEFEE",
                    POP => "POP",
                    MLOAD => "MLOAD",
                    MSTORE => "MSTORE",
                    MSTORE8 => "MSTORE8",
                    SLOAD => "SLOAD",
                    SSTORE => "SSTORE",
                    JUMP => "JUMP",
                    JUMPI => "JUMPI",
                    PC => "PC",
                    MSIZE => "MSIZE",
                    GAS => "GAS",
                    JUMPDEST => "JUMPDEST",
                    PUSH1(_) => unreachable!(),
                    PUSHn(_, _, _) => unreachable!(),
                    DUP(_) => unreachable!(),
                    SWAP(_) => unreachable!(),
                    LOG(_) => unreachable!(),
                    CREATE => "CREATE",
                    CALL => "CALL",
                    CALLCODE => "CALLCODE",
                    RETURN => "RETURN",
                    DELEGATECALL => "DELEGATECALL",
                    CREATE2 => "CREATE2",
                    STATICCALL => "STATICCALL",
                    REVERT => "REVERT",
                    INVALID => "INVALID",
                    SELFDESTRUCT => "SELFDESTRUCT",
                };
                write!(f, "{}", s)
            }
        }
    }
}

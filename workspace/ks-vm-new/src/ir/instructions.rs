#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::types::{Arguments, CaptureSize, NativeId, Offset, VariableId};

use super::constant::Constant;

const BYTE_SIZE: usize = 8;

// Arithmetic (0x00-0x0F)
pub const ADD: u8 = 0x01;
pub const SUB: u8 = 0x02;
pub const MUL: u8 = 0x03;
pub const DIV: u8 = 0x04;
pub const INC: u8 = 0x05;
pub const DEC: u8 = 0x06;

// Comparison (0x10-0x1F)
pub const EQ: u8 = 0x10;
pub const NE: u8 = 0x11;
pub const GT: u8 = 0x12;
pub const GE: u8 = 0x13;
pub const LT: u8 = 0x14; // LITHUANIA LET'S GOOOOOO
pub const LE: u8 = 0x15;

// Logic (0x20-0x2F)
pub const AND: u8 = 0x20;
pub const OR: u8 = 0x21;
pub const NOT: u8 = 0x22;

// Branching (0x30-0x3F)
pub const RET: u8 = 0x30;
pub const JZ: u8 = 0x31; // <u32>
pub const JNZ: u8 = 0x32; // <u32>
pub const JMP: u8 = 0x33; // <u32>

// Stack (0x40-0x4F)
pub const CPY: u8 = 0x40;
pub const CLR: u8 = 0x41;
pub const FREE: u8 = 0x42; // <u32>
pub const CALL: u8 = 0x43; // <u32>
pub const NCALL: u8 = 0x44; // <u32>, <u32>

// MEMORY (0x50-0x5F)
pub const STR: u8 = 0x50;
pub const ASN: u8 = 0x51;
pub const ASV: u8 = 0x52; // <u32>
pub const ASC: u8 = 0x53;
pub const LDV: u8 = 0x54; // <u32>
pub const LDCP: u8 = 0x57; // <u32>
pub const LDFC: u8 = 0x58;
pub const LEN: u8 = 0x59;

// Standard constants (0x60-0x6F)
pub const LDI: u8 = 0x60; // <i64>
pub const LDF: u8 = 0x61; // <f64>
pub const LDS: u8 = 0x62; // <u8>, <&str>
pub const LBT: u8 = 0x63;
pub const LBF: u8 = 0x64;
pub const LDN: u8 = 0x65;
pub const LDFN: u8 = 0x66; // <u32>
pub const LDC: u8 = 0x67;

// Small sized constants (0x70-0x7F)
pub const LDI8: u8 = 0x70; // <u8>
pub const LDI16: u8 = 0x71; // <u16>
pub const LDI32: u8 = 0x72; // <u32>
pub const LDFN8: u8 = 0x73; // <u8>
pub const LDFN16: u8 = 0x74; // <u16>
pub const LDC8: u8 = 0x75; // <u8>
pub const LDC16: u8 = 0x76; // <u16>

// Small sized memory (0x80-0x08F)
pub const LDV8: u8 = 0x80; // <u8>
pub const LDV16: u8 = 0x81; // <u16>
pub const ASV8: u8 = 0x82; // <u8>
pub const ASV16: u8 = 0x83; // <u16>
pub const LDCP8: u8 = 0x84; // <u8>
pub const LDCP16: u8 = 0x85; // <u16>

// Small sized stack (0x90-0x9F)
pub const FREE8: u8 = 0x90; // <u8>
pub const FREE16: u8 = 0x91; // <u16>
pub const CALL8: u8 = 0x92; // <u8>
pub const CALL16: u8 = 0x93; // <u16>

// Small sized branching (0xA0-0xAF)
pub const JZ8: u8 = 0xA0; // <u8>
pub const JZ16: u8 = 0xA1; // <u16>
pub const JNZ8: u8 = 0xA2; // <u8>
pub const JNZ16: u8 = 0xA3; // <u16>
pub const JMP8: u8 = 0xA4; // <u8>
pub const JMP16: u8 = 0xA5; // <u16>

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Add,
    Minus,
    Mul,
    Div,
    Eq,
    GreaterEq,
    Greater,
    LessEq,
    Less,
    NotEq,
    And,
    Or,
    Not,
    Increment,
    Decrement,
    Clone,
    ClearAcc,
    Return,
    Free(usize),
    JumpIfFalse(Offset),
    JumpIfTrue(Offset),
    Jump(Offset),
    Store,
    Assign,
    AssignVariable(VariableId),
    LoadConst(Constant),
    LoadVar(VariableId),
    Call(Arguments),
    CallNative(NativeId, Arguments),
    LoadCapture(VariableId),
    LoadFunction(CaptureSize),
    LoadCollection(usize),
    LoadFromCollection,
    CollectionLen,
    AssignCollection,
}

impl Instruction {
    fn opcode_value_u64(&self, opcode: u8, value: u64) -> Vec<u8> {
        let mut opcode = vec![opcode];
        let mut value = value.to_le_bytes().to_vec();
        opcode.append(&mut value);
        opcode
    }

    fn opcode_value_u64_dynamic(&self, opcode: u8, value: u64) -> Vec<u8> {
        let mut opcode = vec![opcode];
        let value = value.to_le_bytes().to_vec();
        let mut size = 0;

        for current_number in 0..value.len() {
            let byte = value[BYTE_SIZE - current_number - 1];
            if byte != 0 {
                size = BYTE_SIZE - current_number;
                break;
            }
        }

        let mut value = value[0..size].to_vec();

        opcode.push(size as u8);
        opcode.append(&mut value);
        opcode
    }

    fn opcode_value_u32(&self, opcode: u8, value: u32) -> Vec<u8> {
        let mut opcode = vec![opcode];
        let mut size = value.to_le_bytes().to_vec();
        opcode.append(&mut size);
        opcode
    }

    fn load_const(&self, constant: &Constant) -> Vec<u8> {
        match constant {
            Constant::Integer(integer) => self.opcode_value_u64_dynamic(LDI, *integer as u64),
            Constant::Float(float) => self.opcode_value_u64(LDF, float.to_bits()),
            Constant::Boolean(boolean) => {
                if *boolean {
                    return vec![LBT];
                } else {
                    return vec![LBF];
                }
            }
            Constant::String(string) => {
                let mut opcode = vec![LDS];
                let string_length = string.len() as u32;
                let mut string_length = string_length.to_le_bytes().to_vec();
                let mut string_bytes = string.as_str().as_bytes().to_vec();

                opcode.append(&mut string_length);
                opcode.append(&mut string_bytes);

                opcode
            }
            Constant::Null => vec![LDN],
        }
    }

    fn native(&self, native_id: u32, arguments: u32) -> Vec<u8> {
        let mut opcode = vec![NCALL];
        let mut native_id = native_id.to_le_bytes().to_vec();
        let mut arguments = arguments.to_le_bytes().to_vec();

        opcode.append(&mut native_id);
        opcode.append(&mut arguments);

        opcode
    }

    pub fn to_bytes(self) -> Vec<u8> {
        match &self {
            Self::Add => vec![ADD],
            Self::Minus => vec![SUB],
            Self::Mul => vec![MUL],
            Self::Div => vec![DIV],
            Self::Eq => vec![EQ],
            Self::GreaterEq => vec![GE],
            Self::Greater => vec![GT],
            Self::LessEq => vec![LE],
            Self::Less => vec![LT],
            Self::NotEq => vec![NE],
            Self::And => vec![AND],
            Self::Or => vec![OR],
            Self::Not => vec![NOT],
            Self::Increment => vec![INC],
            Self::Decrement => vec![DEC],
            Self::Clone => vec![CPY],
            Self::ClearAcc => vec![CLR],
            Self::Return => vec![RET],
            Self::Free(size) => self.opcode_value_u32(FREE, *size as u32),
            Self::JumpIfFalse(offset) => self.opcode_value_u32(JZ, *offset as u32),
            Self::JumpIfTrue(offset) => self.opcode_value_u32(JNZ, *offset as u32),
            Self::Jump(offset) => self.opcode_value_u32(JMP, *offset as u32),
            Self::Store => vec![STR],
            Self::Assign => vec![ASN],
            Self::AssignVariable(variable_id) => self.opcode_value_u32(ASV, *variable_id),
            Self::AssignCollection => vec![ASC],
            Self::LoadConst(constant) => self.load_const(constant),
            Self::LoadVar(variable_id) => self.opcode_value_u32(LDV, *variable_id),
            Self::Call(arguments) => self.opcode_value_u32(CALL, *arguments as u32),
            Self::CallNative(native_id, arguments) => {
                self.native(*native_id as u32, *arguments as u32)
            }
            Self::LoadCapture(captured) => self.opcode_value_u32(LDCP, *captured),
            Self::LoadFunction(size) => self.opcode_value_u32(LDFN, *size as u32),
            Self::LoadCollection(size) => self.opcode_value_u32(LDC, *size as u32),
            Self::LoadFromCollection => vec![LDFC],
            Self::CollectionLen => vec![LEN],
        }
    }
}

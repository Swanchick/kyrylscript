#[cfg(not(feature = "std"))]
use alloc::vec;

use crate::types::{Arguments, CaptureSize, NativeId, Offset, VariableId};

use super::constant::Constant;

// Arithmetic (0x00-0x0F)
const ADD: u8 = 0x00;
const SUB: u8 = 0x01;
const MUL: u8 = 0x02;
const DIV: u8 = 0x03;
const INC: u8 = 0x04;
const DEC: u8 = 0x05;

// Comparison (0x10-0x1F)
const EQ: u8 = 0x10;
const NE: u8 = 0x11;
const GT: u8 = 0x12;
const GE: u8 = 0x13;
const LT: u8 = 0x14; // LITHUANIA LET'S GOOOOOO
const LE: u8 = 0x15;

// Logic (0x20-0x2F)
const AND: u8 = 0x20;
const OR: u8 = 0x21;
const NOT: u8 = 0x22;

// Control (0x30-0x3F)
const RET: u8 = 0x30;
const JZ: u8 = 0x31;
const JNZ: u8 = 0x32;
const JMP: u8 = 0x33;

// Stack (0x40-0x4F)
const CPY: u8 = 0x40;
const CLR: u8 = 0x41;
const FREE: u8 = 0x42;
const CALL: u8 = 0x43;
const NCALL: u8 = 0x44;

// Constants (0x50-0x5F)
const LDI: u8 = 0x50;
const LDF: u8 = 0x51;
const LDS: u8 = 0x52;
const LBT: u8 = 0x53;
const LBF: u8 = 0x54;
const LDN: u8 = 0x55;
const LDFN: u8 = 0x56;
const LDC: u8 = 0x57;

// MEMORY (0x60-0x6F)
const STR: u8 = 0x60;
const ASN: u8 = 0x61;
const ASV: u8 = 0x62;
const ASC: u8 = 0x63;
const LDV: u8 = 0x64;
const LDCP: u8 = 0x65;
const LDFC: u8 = 0x66;
const LEN: u8 = 0x67;

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
    Call,
    CallNative(NativeId, Arguments),
    LoadCapture(VariableId),
    LoadFunction(CaptureSize),
    LoadCollection(usize),
    LoadFromCollection,
    CollectionLen,
    AssignCollection,
}

impl Instruction {
    fn opcode_value(&self, opcode: u8, value: u64) -> Vec<u8> {
        let mut opcode = vec![opcode];
        let mut size = value.to_le_bytes().to_vec();
        opcode.append(&mut size);
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
            Constant::Integer(integer) => self.opcode_value(LDI, *integer as u64),
            Constant::Float(float) => self.opcode_value(LDF, float.to_bits()),
            Constant::Boolean(boolean) => {
                if *boolean {
                    vec![LBT]
                } else {
                    vec![LBF]
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
            Self::Free(size) => self.opcode_value(FREE, *size as u64),
            Self::JumpIfFalse(offset) => self.opcode_value_u32(JNZ, *offset as u32),
            Self::JumpIfTrue(offset) => self.opcode_value_u32(JZ, *offset as u32),
            Self::Jump(offset) => self.opcode_value_u32(JMP, *offset as u32),
            Self::Store => vec![STR],
            Self::Assign => vec![ASN],
            Self::AssignVariable(variable_id) => self.opcode_value(ASV, *variable_id),
            Self::AssignCollection => vec![ASC],
            Self::LoadConst(constant) => self.load_const(constant),
            Self::LoadVar(variable_id) => self.opcode_value(LDV, *variable_id),
            Self::Call => vec![CALL],
            Self::CallNative(native_id, arguments) => {
                self.native(*native_id as u32, *arguments as u32)
            }
            Self::LoadCapture(captured) => self.opcode_value(LDCP, *captured),
            Self::LoadFunction(size) => self.opcode_value_u32(LDFN, *size as u32),
            Self::LoadCollection(size) => self.opcode_value_u32(LDC, *size as u32),
            Self::LoadFromCollection => vec![LDFC],
            Self::CollectionLen => vec![LEN],
        }
    }
}

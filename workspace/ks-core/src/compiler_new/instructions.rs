use crate::compiler_new::types::VariableId;

use super::constant::Constant;

const SINGLE_INSTRUCTION: usize = 1;
const BYTE_INSTRUCTION: usize = 2;
const WORD_INSTRUCTION: usize = 3;
const DWORD_INSTRUCTION: usize = 5;
const QWORD_INSTRUCTION: usize = 9;

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
    JumpIfFalse(i32),
    JumpIfTrue(i32),
    Jump(i32),
    Store,
    Assign,
    AssignVariable(VariableId),
    LoadConst(Constant),
    LoadVar(VariableId),
    Call(u32),
    CallNative(u32, u32),
    LoadCapture(u32),
    LoadFunction(usize),
    LoadCollection(usize),
    LoadFromCollection,
    CollectionLen,
    AssignCollection,
}

impl Instruction {
    fn compressed_u32(&self, number: u32) -> usize {
        if let Ok(_) = u8::try_from(number) {
            BYTE_INSTRUCTION
        } else if let Ok(_) = u16::try_from(number) {
            WORD_INSTRUCTION
        } else {
            DWORD_INSTRUCTION
        }
    }

    fn compressed_i32(&self, number: i32) -> usize {
        if let Ok(_) = i8::try_from(number) {
            BYTE_INSTRUCTION
        } else if let Ok(_) = i16::try_from(number) {
            WORD_INSTRUCTION
        } else {
            DWORD_INSTRUCTION
        }
    }

    fn compressed_i64(&self, number: i64) -> usize {
        if let Ok(_) = i8::try_from(number) {
            BYTE_INSTRUCTION
        } else if let Ok(_) = i16::try_from(number) {
            WORD_INSTRUCTION
        } else if let Ok(_) = i32::try_from(number) {
            DWORD_INSTRUCTION
        } else {
            QWORD_INSTRUCTION
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Add => SINGLE_INSTRUCTION,
            Self::Minus => SINGLE_INSTRUCTION,
            Self::Mul => SINGLE_INSTRUCTION,
            Self::Div => SINGLE_INSTRUCTION,
            Self::Eq => SINGLE_INSTRUCTION,
            Self::GreaterEq => SINGLE_INSTRUCTION,
            Self::Greater => SINGLE_INSTRUCTION,
            Self::LessEq => SINGLE_INSTRUCTION,
            Self::Less => SINGLE_INSTRUCTION,
            Self::NotEq => SINGLE_INSTRUCTION,
            Self::And => SINGLE_INSTRUCTION,
            Self::Or => SINGLE_INSTRUCTION,
            Self::Not => SINGLE_INSTRUCTION,
            Self::Increment => SINGLE_INSTRUCTION,
            Self::Decrement => SINGLE_INSTRUCTION,
            Self::Clone => SINGLE_INSTRUCTION,
            Self::ClearAcc => SINGLE_INSTRUCTION,
            Self::Return => SINGLE_INSTRUCTION,
            Self::Free(size) => self.compressed_u32(*size as u32),
            Self::JumpIfFalse(offset) => self.compressed_i32(*offset),
            Self::JumpIfTrue(offset) => self.compressed_i32(*offset),
            Self::Jump(offset) => self.compressed_i32(*offset),
            Self::Store => SINGLE_INSTRUCTION,
            Self::Assign => SINGLE_INSTRUCTION,
            Self::AssignVariable(variable_id) => self.compressed_u32(*variable_id),
            Self::AssignCollection => SINGLE_INSTRUCTION,
            Self::LoadVar(variable_id) => self.compressed_u32(*variable_id),
            Self::Call(arguments) => self.compressed_u32(*arguments),
            Self::LoadConst(Constant::Integer(integer)) => self.compressed_i64(*integer),
            Self::LoadConst(Constant::Float(_)) => QWORD_INSTRUCTION,
            Self::LoadConst(Constant::Boolean(_)) => SINGLE_INSTRUCTION,
            Self::LoadConst(Constant::Null) => SINGLE_INSTRUCTION,
            Self::LoadConst(Constant::String(string)) => DWORD_INSTRUCTION + string.len(),
            Self::CallNative(_, _) => QWORD_INSTRUCTION,
            Self::LoadCapture(captured) => self.compressed_u32(*captured),
            Self::LoadFunction(size) => self.compressed_u32(*size as u32),
            Self::LoadCollection(size) => self.compressed_u32(*size as u32),
            Self::LoadFromCollection => SINGLE_INSTRUCTION,
            Self::CollectionLen => SINGLE_INSTRUCTION,
        }
    }
}

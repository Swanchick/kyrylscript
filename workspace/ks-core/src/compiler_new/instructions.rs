use ks_vm_new::ir::instructions::{
    ADD, AND, ASC, ASN, ASV, ASV8, ASV16, BYTE_SIZE, CALL, CALL8, CALL16, CLR, CPY, DEC, DIV, EQ,
    FREE, FREE8, FREE16, GE, GT, INC, JMP, JMP8, JMP16, JNZ, JNZ8, JNZ16, JZ, JZ8, JZ16, LBF, LBT,
    LDC, LDC8, LDC16, LDCP, LDCP8, LDCP16, LDF, LDFC, LDFN, LDFN8, LDFN16, LDI, LDI8, LDI16, LDI32,
    LDN, LDS, LDV, LDV8, LDV16, LE, LEN, LT, MUL, NCALL, NE, NOT, OR, RET, STR, SUB,
};

use crate::compiler_new::types::VariableId;

use super::constant::Constant;

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
            Constant::Integer(integer) => self.compressed_i64(LDI8, LDI16, LDI32, LDI, *integer),
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

    fn compressed_u32(
        &self,
        instruction_8: u8,
        instruction_16: u8,
        instruction_32: u8,
        number: u32,
    ) -> Vec<u8> {
        let mut opcode: Vec<u8> = vec![];

        if let Ok(v) = u8::try_from(number) {
            opcode.push(instruction_8);
            opcode.append(&mut v.to_le_bytes().to_vec());
        } else if let Ok(v) = u16::try_from(number) {
            opcode.push(instruction_16);
            opcode.append(&mut v.to_le_bytes().to_vec());
        } else {
            opcode.push(instruction_32);
            opcode.append(&mut number.to_le_bytes().to_vec());
        }

        opcode
    }

    fn compressed_i32(
        &self,
        instruction_8: u8,
        instruction_16: u8,
        instruction_32: u8,
        number: i32,
    ) -> Vec<u8> {
        let mut opcode: Vec<u8> = vec![];

        if let Ok(v) = i8::try_from(number) {
            opcode.push(instruction_8);
            opcode.append(&mut v.to_le_bytes().to_vec());
        } else if let Ok(v) = i16::try_from(number) {
            opcode.push(instruction_16);
            opcode.append(&mut v.to_le_bytes().to_vec());
        } else {
            opcode.push(instruction_32);
            opcode.append(&mut number.to_le_bytes().to_vec());
        }

        opcode
    }

    fn compressed_i64(
        &self,
        instruction_8: u8,
        instruction_16: u8,
        instruction_32: u8,
        instruction_64: u8,
        number: i64,
    ) -> Vec<u8> {
        let mut opcode: Vec<u8> = vec![];

        if let Ok(v) = i8::try_from(number) {
            opcode.push(instruction_8);
            opcode.append(&mut v.to_le_bytes().to_vec());
        } else if let Ok(v) = i16::try_from(number) {
            opcode.push(instruction_16);
            opcode.append(&mut v.to_le_bytes().to_vec());
        } else if let Ok(v) = i32::try_from(number) {
            opcode.push(instruction_32);
            opcode.append(&mut v.to_le_bytes().to_vec());
        } else {
            opcode.push(instruction_64);
            opcode.append(&mut number.to_le_bytes().to_vec());
        }

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
            Self::Free(size) => self.compressed_u32(FREE8, FREE16, FREE, *size as u32),
            Self::JumpIfFalse(offset) => self.compressed_i32(JZ8, JZ16, JZ, *offset),
            Self::JumpIfTrue(offset) => self.compressed_i32(JNZ8, JNZ16, JNZ, *offset),
            Self::Jump(offset) => self.compressed_i32(JMP8, JMP16, JMP, *offset),
            Self::Store => vec![STR],
            Self::Assign => vec![ASN],
            Self::AssignVariable(variable_id) => {
                self.compressed_u32(ASV8, ASV16, ASV, *variable_id)
            }
            Self::AssignCollection => vec![ASC],
            Self::LoadConst(constant) => self.load_const(constant),
            Self::LoadVar(variable_id) => self.compressed_u32(LDV8, LDV16, LDV, *variable_id),
            Self::Call(arguments) => self.compressed_u32(CALL8, CALL16, CALL, *arguments),
            Self::CallNative(native_id, arguments) => self.native(*native_id, *arguments),
            Self::LoadCapture(captured) => self.compressed_u32(LDCP8, LDCP16, LDCP, *captured),
            Self::LoadFunction(size) => self.compressed_u32(LDFN8, LDFN16, LDFN, *size as u32),
            Self::LoadCollection(size) => self.compressed_u32(LDC8, LDC16, LDC, *size as u32),
            Self::LoadFromCollection => vec![LDFC],
            Self::CollectionLen => vec![LEN],
        }
    }
}

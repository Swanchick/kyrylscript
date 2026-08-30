use ks_vm_new::ir::instructions::{
    ADD, AND, ASC, ASN, ASV, ASV8, ASV16, CALL, CALL8, CALL16, CLR, CPY, DEC, DIV, EQ, FREE, FREE8,
    FREE16, GE, GT, INC, JMP, JMP8, JMP16, JNZ, JNZ8, JNZ16, JZ, JZ8, JZ16, LBF, LBT, LDC, LDC8,
    LDC16, LDCP, LDCP8, LDCP16, LDF, LDFC, LDFN, LDI, LDI8, LDI16, LDI32, LDN, LDS, LDV, LDV8,
    LDV16, LE, LEN, LT, MUL, NCALL, NE, NOT, OR, RET, STR, SUB,
};

use super::constant::Constant;
use super::instructions::Instruction;

pub struct Serializer {
    instructions: Vec<Instruction>,
    instruction_positions: Vec<usize>,
}

impl Serializer {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            instruction_positions: Vec::new(),
        }
    }

    fn opcode_value_u64(&self, opcode: u8, value: u64) -> Vec<u8> {
        let mut opcode = vec![opcode];
        let mut value = value.to_le_bytes().to_vec();
        opcode.append(&mut value);
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

    fn dual_instruction(&self, instruction: u8, parameter1: u32, parameter2: u32) -> Vec<u8> {
        let mut opcode = vec![instruction];
        let mut parameter1 = parameter1.to_le_bytes().to_vec();
        let mut parameter2 = parameter2.to_le_bytes().to_vec();

        opcode.append(&mut parameter1);
        opcode.append(&mut parameter2);

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

    pub fn prepare_map(&mut self) {
        let mut pc = 0;

        for instruction in &self.instructions {
            self.instruction_positions.push(pc);
            pc += instruction.size();
        }
    }

    pub fn jump(
        &self,
        instruction_8: u8,
        instruction_16: u8,
        instruction_32: u8,
        index: usize,
        offset: i32,
    ) -> Vec<u8> {
        let difference = index as i32 + offset;

        let jump_index = self.instruction_positions[index] as i32;
        let instruction_jump = self.instruction_positions[difference as usize] as i32;
        let actual_distance = instruction_jump - jump_index;

        self.compressed_i32(
            instruction_8,
            instruction_16,
            instruction_32,
            actual_distance,
        )
    }

    #[inline]
    fn convert_pointer(&self, pointer: u32) -> u32 {
        self.instruction_positions[pointer as usize] as u32
    }

    pub fn serialize_instruction(&self, index: usize, instruction: &Instruction) -> Vec<u8> {
        match instruction {
            Instruction::Add => vec![ADD],
            Instruction::Minus => vec![SUB],
            Instruction::Mul => vec![MUL],
            Instruction::Div => vec![DIV],
            Instruction::Eq => vec![EQ],
            Instruction::GreaterEq => vec![GE],
            Instruction::Greater => vec![GT],
            Instruction::LessEq => vec![LE],
            Instruction::Less => vec![LT],
            Instruction::NotEq => vec![NE],
            Instruction::And => vec![AND],
            Instruction::Or => vec![OR],
            Instruction::Not => vec![NOT],
            Instruction::Increment => vec![INC],
            Instruction::Decrement => vec![DEC],
            Instruction::Clone => vec![CPY],
            Instruction::ClearAcc => vec![CLR],
            Instruction::Return => vec![RET],
            Instruction::Free(size) => self.compressed_u32(FREE8, FREE16, FREE, *size as u32),
            Instruction::JumpIfFalse(offset) => self.jump(JZ8, JZ16, JZ, index, *offset),
            Instruction::JumpIfTrue(offset) => self.jump(JNZ8, JNZ16, JNZ, index, *offset),
            Instruction::Jump(offset) => self.jump(JMP8, JMP16, JMP, index, *offset),
            Instruction::Store => vec![STR],
            Instruction::Assign => vec![ASN],
            Instruction::AssignVariable(variable_id) => {
                self.compressed_u32(ASV8, ASV16, ASV, *variable_id)
            }
            Instruction::AssignCollection => vec![ASC],
            Instruction::LoadConst(constant) => self.load_const(constant),
            Instruction::LoadVar(variable_id) => {
                self.compressed_u32(LDV8, LDV16, LDV, *variable_id)
            }
            Instruction::Call(arguments) => self.compressed_u32(CALL8, CALL16, CALL, *arguments),
            Instruction::CallNative(native_id, arguments) => {
                self.dual_instruction(NCALL, *native_id, *arguments)
            }
            Instruction::LoadCapture(captured) => {
                self.compressed_u32(LDCP8, LDCP16, LDCP, *captured)
            }
            Instruction::LoadFunction(pointer, size) => {
                self.dual_instruction(LDFN, self.convert_pointer(*pointer), *size as u32)
            }
            Instruction::LoadCollection(size) => {
                self.compressed_u32(LDC8, LDC16, LDC, *size as u32)
            }
            Instruction::LoadFromCollection => vec![LDFC],
            Instruction::CollectionLen => vec![LEN],
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut program = Vec::<u8>::new();
        for (index, instruction) in self.instructions.iter().enumerate() {
            program.append(&mut self.serialize_instruction(index, instruction));
        }
        program
    }
}

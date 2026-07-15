#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::ir::instructions::{
    ADD, AND, CALL, CLR, CPY, DEC, DIV, EQ, FREE, GE, GT, INC, JMP, JNZ, JZ, LDI, LE, LT, MUL,
    NCALL, NE, NOT, OR, RET, SUB,
};
use crate::{Constant, Instruction, Program};
use crate::{VMError, VMResult};

const SINGLE_INSTRUCTION_SIZE: usize = 1;
const NUMBER_U32_SIZE: usize = 4;
const NUMBER_U64_SIZE: usize = 8;

pub struct Deserialize {
    buffer: Vec<u8>,
    instructions: Vec<Instruction>,
    pc: usize,
}

impl From<Vec<u8>> for Deserialize {
    fn from(buffer: Vec<u8>) -> Self {
        Self {
            buffer,
            instructions: Vec::new(),
            pc: 0,
        }
    }
}

impl Deserialize {
    fn add(&mut self, instruction: Instruction) -> VMResult<()> {
        self.instructions.push(instruction);
        self.pc = self.pc.saturating_add(1);

        Ok(())
    }

    fn parse_u32(&mut self, pc: usize) -> VMResult<u32> {
        let bytes = &self.buffer[pc..pc + NUMBER_U32_SIZE];
        let bytes = bytes
            .try_into()
            .map_err(|_| VMError::from("Invalid u32 number"))?;

        let number = u32::from_le_bytes(bytes);

        Ok(number)
    }

    fn parse_u64(&mut self, pc: usize) -> VMResult<u64> {
        let bytes = &self.buffer[pc..pc + NUMBER_U64_SIZE];
        let bytes = bytes
            .try_into()
            .map_err(|_| VMError::from("Invalid u32 number"))?;

        let number = u64::from_le_bytes(bytes);

        Ok(number)
    }

    fn add_u32(&mut self, instruction: impl Fn(u32) -> Instruction) -> VMResult<()> {
        let number = self.parse_u32(self.pc + SINGLE_INSTRUCTION_SIZE)?;
        let instruction = instruction(number);
        self.add(instruction)?;

        self.pc = self.pc.saturating_add(NUMBER_U32_SIZE);

        Ok(())
    }

    fn add_u64(&mut self, instruction: impl Fn(u64) -> Instruction) -> VMResult<()> {
        let number = self.parse_u64(self.pc + SINGLE_INSTRUCTION_SIZE)?;
        let instruction = instruction(number);
        self.add(instruction)?;
        self.pc = self.pc.saturating_add(NUMBER_U64_SIZE);

        Ok(())
    }

    fn ncall(&mut self) -> VMResult<()> {
        let native_id = self.parse_u32(self.pc + SINGLE_INSTRUCTION_SIZE)?;
        self.pc = self.pc.saturating_add(NUMBER_U32_SIZE);
        let arguments = self.parse_u32(self.pc + SINGLE_INSTRUCTION_SIZE)?;
        self.pc = self.pc.saturating_add(NUMBER_U32_SIZE);

        self.add(Instruction::CallNative(
            native_id as usize,
            arguments as usize,
        ))?;

        Ok(())
    }

    pub fn deserialize(mut self) -> VMResult<Program> {
        while let Some(opcode) = self.buffer.get(self.pc) {
            match *opcode {
                ADD => self.add(Instruction::Add),
                SUB => self.add(Instruction::Minus),
                MUL => self.add(Instruction::Mul),
                DIV => self.add(Instruction::Div),
                INC => self.add(Instruction::Increment),
                DEC => self.add(Instruction::Decrement),
                EQ => self.add(Instruction::Eq),
                NE => self.add(Instruction::NotEq),
                GT => self.add(Instruction::Greater),
                GE => self.add(Instruction::GreaterEq),
                LT => self.add(Instruction::Less),
                LE => self.add(Instruction::LessEq),
                AND => self.add(Instruction::And),
                OR => self.add(Instruction::Or),
                NOT => self.add(Instruction::Not),
                RET => self.add(Instruction::Return),
                JZ => self.add_u32(|num| Instruction::JumpIfFalse(num as i32)),
                JNZ => self.add_u32(|num| Instruction::JumpIfTrue(num as i32)),
                JMP => self.add_u32(|num| Instruction::Jump(num as i32)),
                CPY => self.add(Instruction::Clone),
                CLR => self.add(Instruction::ClearAcc),
                FREE => self.add_u32(|num| Instruction::Free(num as usize)),
                CALL => self.add(Instruction::Call),
                NCALL => self.ncall(),
                LDI => self.add_u64(|num| Instruction::LoadConst(Constant::Integer(num as i64))),
                _ => Err(VMError::from("Invalid opcode")),
            }?;
        }

        let program = Program::from(self.instructions);

        Ok(program)
    }
}

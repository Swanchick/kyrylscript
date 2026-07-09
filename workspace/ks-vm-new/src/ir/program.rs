#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::{VMError, VMResult};

use super::instructions::ADD;

use super::instructions::Instruction;

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn add(&mut self, instruction: Instruction) -> VMResult<()> {
        self.instructions.push(instruction);
        Ok(())
    }

    pub fn deserialize(buffer: Vec<u8>) -> VMResult<Self> {
        let mut program = Program::new(Vec::new());

        for opcode in buffer {
            match opcode {
                ADD => program.add(Instruction::Add),
                _ => Err(VMError::from("Invalid opcode")),
            }?;
        }

        Ok(program)
    }

    pub fn serialize(self) -> Vec<u8> {
        let mut out = Vec::<u8>::new();
        for instruction in self.instructions {
            let mut bytes = instruction.to_bytes();
            out.append(&mut bytes);
        }

        out
    }
}

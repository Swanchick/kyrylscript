#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

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

    pub fn serialize<'a>(self) -> Vec<u8> {
        let mut out = Vec::<u8>::new();

        for instruction in self.instructions {}

        todo!()
    }
}

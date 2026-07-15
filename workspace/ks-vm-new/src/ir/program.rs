#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::VMResult;
use crate::ir::deserialize::Deserialize;

use super::instructions::Instruction;

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl From<Vec<Instruction>> for Program {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}

impl Program {
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn deserialize(buffer: Vec<u8>) -> VMResult<Self> {
        let deserialize = Deserialize::from(buffer);
        let program = deserialize.deserialize()?;

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

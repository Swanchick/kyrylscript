#[cfg(not(feature = "std"))]
use alloc::vec::Box;

use super::instructions::Instruction;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Box<[u8]>,
}

impl From<Vec<u8>> for Program {
    fn from(value: Vec<u8>) -> Self {
        Program {
            instructions: value.into_boxed_slice(),
        }
    }
}

impl Program {
    pub fn serialize(instructions: Vec<Instruction>) -> Program {
        let mut out = Vec::new();
        for instruction in instructions {
            let mut bytes = instruction.to_bytes();
            out.append(&mut bytes);
        }

        Program::from(out)
    }
}

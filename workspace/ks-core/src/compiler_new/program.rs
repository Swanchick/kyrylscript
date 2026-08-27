use crate::compiler_new::serializer::Serializer;

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
        let mut serializer = Serializer::new(instructions);
        serializer.prepare_map();

        let out = serializer.serialize();

        Program::from(out)
    }

    pub fn as_bytes(self) -> Box<[u8]> {
        self.instructions
    }
}

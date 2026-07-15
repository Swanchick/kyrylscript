use crate::ir::instructions::{ADD, DEC, DIV, EQ, GT, INC, MUL, NE, SUB};
use crate::{Instruction, Program};
use crate::{VMError, VMResult};

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
                _ => Err(VMError::from("Invalid opcode")),
            }?;
        }

        let program = Program::from(self.instructions);

        Ok(program)
    }
}

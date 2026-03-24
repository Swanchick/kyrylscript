use ks_global::utils::ks_result::KsResult;

use super::types::Pointer;
use super::vgs::VGS;
use super::{Constant, Instruction};

pub struct Runner {
    program_counter: Pointer,
}

impl Runner {
    pub fn new() -> Self {
        Self { program_counter: 0 }
    }

    fn load_const(&mut self, constant: Constant) -> KsResult<()> {
        todo!()
    }

    pub fn step(&mut self, instruction: Instruction, vgs: &mut VGS) -> KsResult<()> {
        match instruction {
            Instruction::LoadConst(constant) => self.load_const(constant)?,
            _ => todo!(),
        }

        Ok(())
    }

    pub fn program_counter(&self) -> Pointer {
        self.program_counter
    }
}

use ks_global::utils::ks_result::KsResult;

use super::Instruction;
use super::types::Pointer;
use super::vgs::VGS;

pub struct Runner {
    program_counter: Pointer,
}

impl From<Pointer> for Runner {
    fn from(program_counter: Pointer) -> Self {
        Self { program_counter }
    }
}

impl Runner {
    pub fn new() -> Self {
        Self { program_counter: 0 }
    }

    pub fn step(&mut self, instruction: Instruction, vgs: &mut VGS) -> KsResult<()> {
        Ok(())
    }

    pub fn program_counter(&self) -> Pointer {
        self.program_counter
    }
}

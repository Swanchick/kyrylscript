use ks_global::utils::ks_result::KsResult;

use super::types::{Pointer, Stack};
use super::{Constant, Instruction};
use crate::gvs::{GVS, Variable};

pub struct Runner {
    program_counter: Pointer,
    pub acc: Stack,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            acc: Vec::new(),
        }
    }

    fn load_const(&mut self, gvs: &mut GVS, constant: Constant) -> KsResult<()> {
        let variable = match constant {
            Constant::Null => Variable::null(),
            Constant::Integer(value) => Variable::from(value),
            Constant::Float(value) => Variable::from(value),
            Constant::Boolean(value) => Variable::from(value),
            _ => todo!(),
        };

        let storage_id = gvs.store(variable);
        self.acc.push(storage_id);

        Ok(())
    }

    pub fn step(&mut self, instruction: Instruction, gvs: &mut GVS) -> KsResult<()> {
        match instruction {
            Instruction::LoadConst(constant) => self.load_const(gvs, constant),
            _ => todo!(),
        }?;

        Ok(())
    }

    pub fn program_counter(&self) -> Pointer {
        self.program_counter
    }
}

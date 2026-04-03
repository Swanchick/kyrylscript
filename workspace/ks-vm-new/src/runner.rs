use ks_global::utils::ks_result::KsResult;

use super::types::{Pointer, Slot, Stack};
use super::{Constant, Instruction};
use crate::gvs::{GVS, Variable};

pub struct Runner {
    pub program_counter: Pointer,
    pub acc: Stack,
    pub stack: Stack,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            acc: Vec::new(),
            stack: Vec::new(),
        }
    }

    fn step(&mut self) {
        self.program_counter += 1;
    }

    fn load_const_string(&mut self, gvs: &mut GVS, string: String) -> Variable {
        let collection_id = gvs.collection_store_string(string);

        Variable::string(collection_id)
    }

    fn load_const(&mut self, gvs: &mut GVS, constant: Constant) -> KsResult<()> {
        let variable = match constant {
            Constant::Null => Variable::null(),
            Constant::Integer(value) => Variable::from(value),
            Constant::Float(value) => Variable::from(value),
            Constant::Boolean(value) => Variable::from(value),
            Constant::String(string) => self.load_const_string(gvs, string),
        };

        let storage_id = gvs.store(variable);
        self.acc.push(storage_id);

        self.step();

        Ok(())
    }

    fn load_var(&mut self, gvs: &mut GVS, slot: Slot) -> KsResult<()> {
        Ok(())
    }

    pub fn run(&mut self, instruction: Instruction, gvs: &mut GVS) -> KsResult<()> {
        match instruction {
            Instruction::LoadConst(constant) => self.load_const(gvs, constant),
            Instruction::LoadVar(slot) => self.load_var(gvs, slot),
            _ => todo!(),
        }?;

        Ok(())
    }

    pub fn program_counter(&self) -> Pointer {
        self.program_counter
    }
}

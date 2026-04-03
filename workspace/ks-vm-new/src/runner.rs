use std::ops::Neg;

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use super::types::{Offset, Pointer, Slot, Stack, StorageId};
use super::{Constant, Instruction};
use crate::gvs::{GVS, Variable};

#[derive(Debug)]
pub struct Runner {
    pub program_counter: Pointer,
    pub acc: Stack,
    pub stack: Stack,
    pub prevent_step: bool,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            acc: Vec::new(),
            stack: Vec::new(),
            prevent_step: false,
        }
    }

    fn storage_by_slot(&self, slot: Slot) -> KsResult<StorageId> {
        if let Some(storage_id) = self.stack.get(slot as usize) {
            Ok(*storage_id)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot get storage_id by slot {}",
                slot
            )))
        }
    }

    fn step(&mut self) {
        if !self.prevent_step {
            self.program_counter += 1;
        }
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

        Ok(())
    }

    fn load_var(&mut self, gvs: &mut GVS, slot: Slot) -> KsResult<()> {
        let storage_id = self.storage_by_slot(slot)?;
        gvs.storeage_add_owner(storage_id)?;
        self.acc.push(storage_id);

        Ok(())
    }

    fn jump(&mut self, offset: Offset) -> KsResult<()> {
        let offset = offset - 1;

        self.program_counter = if offset < 0 {
            self.program_counter.saturating_sub(offset.neg() as usize)
        } else {
            self.program_counter.saturating_add(offset as usize)
        };

        Ok(())
    }

    pub fn run(&mut self, instruction: Instruction, gvs: &mut GVS) -> KsResult<()> {
        match instruction {
            Instruction::LoadConst(constant) => self.load_const(gvs, constant),
            Instruction::LoadVar(slot) => self.load_var(gvs, slot),
            Instruction::Jump(offset) => self.jump(offset),
            _ => todo!(),
        }?;

        self.step();

        Ok(())
    }

    pub fn program_counter(&self) -> Pointer {
        self.program_counter
    }
}

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use super::types::{Offset, Pointer, Slot, Stack, StorageId};
use super::{Constant, Instruction};
use crate::gvs::variable::{FLOAT_TYPE, INT_TYPE, STRING_TYPE};
use crate::gvs::{GVS, Variable};
use crate::types::CollectionId;

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

    fn storage(&self, slot: Slot) -> KsResult<StorageId> {
        if let Some(storage_id) = self.stack.get(slot as usize) {
            Ok(*storage_id)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot get storage_id by slot {}",
                slot
            )))
        }
    }

    fn acc_push(&mut self, gvs: &mut GVS, variable: Variable) -> KsResult<()> {
        let storage_id = gvs.store(variable);
        gvs.storage_add_owner(storage_id)?;

        self.acc.push(storage_id);

        Ok(())
    }

    fn acc_pop<'a>(&mut self, gvs: &'a mut GVS) -> KsResult<&'a Variable> {
        if let Some(storage_id) = self.acc.pop() {
            gvs.storage_remove_owner(storage_id)?;
            let variable = gvs.variable(storage_id)?;

            Ok(variable)
        } else {
            Err(KsError::runtime("No Varialbe in ACC"))
        }
    }

    fn step(&mut self) {
        if !self.prevent_step {
            self.program_counter += 1;
        }

        self.prevent_step = false;
    }

    fn load_const_string(gvs: &mut GVS, string: String) -> Variable {
        let collection_id = gvs.collection_store_string(string);

        Variable::string(collection_id)
    }

    fn load_const(&mut self, gvs: &mut GVS, constant: Constant) -> KsResult<()> {
        let variable = match constant {
            Constant::Null => Variable::null(),
            Constant::Integer(value) => Variable::from(value),
            Constant::Float(value) => Variable::from(value),
            Constant::Boolean(value) => Variable::from(value),
            Constant::String(string) => Self::load_const_string(gvs, string),
        };

        self.acc_push(gvs, variable)?;

        Ok(())
    }

    fn load_var(&mut self, gvs: &mut GVS, slot: Slot) -> KsResult<()> {
        let storage_id = self.storage(slot)?;
        gvs.storage_add_owner(storage_id)?;
        self.acc.push(storage_id);

        Ok(())
    }

    fn jump(&mut self, offset: Offset) -> KsResult<()> {
        self.program_counter = if offset < 0 {
            self.program_counter
                .saturating_sub(offset.unsigned_abs() as usize)
        } else {
            self.program_counter.saturating_add(offset as usize)
        };

        self.prevent_step = true;

        Ok(())
    }

    fn add_strings(gvs: &mut GVS, left: CollectionId, right: CollectionId) -> KsResult<u64> {
        let mut left = gvs.collection_string(left)?.to_string();
        let right = gvs.collection_string(right)?;

        left.push_str(right);
        let collection_id = gvs.collection_store_string(left);

        Ok(collection_id)
    }

    fn add(&mut self, gvs: &mut GVS) -> KsResult<()> {
        let right = self.acc_pop(gvs)?.clone();
        let left = self.acc_pop(gvs)?.clone();

        let variable = match (left.value_type, right.value_type) {
            (INT_TYPE, INT_TYPE) => Variable::from(left.value as i64 + right.value as i64),
            (INT_TYPE, FLOAT_TYPE) | (FLOAT_TYPE, INT_TYPE) | (FLOAT_TYPE, FLOAT_TYPE) => {
                Variable::from(left.as_f64()? + right.as_f64()?)
            }
            (STRING_TYPE, STRING_TYPE) => {
                let collection_id = Self::add_strings(gvs, left.value, right.value)?;
                Variable::string(collection_id)
            }
            _ => unreachable!(),
        };

        self.acc_push(gvs, variable)?;

        Ok(())
    }

    fn minus(&mut self, gvs: &mut GVS) -> KsResult<()> {
        let right = self.acc_pop(gvs)?.clone();
        let left = self.acc_pop(gvs)?.clone();

        let variable = match (left.value_type, right.value_type) {
            (INT_TYPE, INT_TYPE) => Variable::from(left.value as i64 - right.value as i64),
            (INT_TYPE, FLOAT_TYPE) | (FLOAT_TYPE, INT_TYPE) | (FLOAT_TYPE, FLOAT_TYPE) => {
                Variable::from(left.as_f64()? - right.as_f64()?)
            }
            _ => unreachable!(),
        };

        self.acc_push(gvs, variable)?;

        Ok(())
    }

    fn mul(&mut self, gvs: &mut GVS) -> KsResult<()> {
        let right = self.acc_pop(gvs)?.clone();
        let left = self.acc_pop(gvs)?.clone();

        let variable = match (left.value_type, right.value_type) {
            (INT_TYPE, INT_TYPE) => Variable::from(left.value as i64 * right.value as i64),
            (INT_TYPE, FLOAT_TYPE) | (FLOAT_TYPE, INT_TYPE) | (FLOAT_TYPE, FLOAT_TYPE) => {
                Variable::from(left.as_f64()? * right.as_f64()?)
            }
            _ => unreachable!(),
        };

        self.acc_push(gvs, variable)?;

        Ok(())
    }

    fn div(&mut self, gvs: &mut GVS) -> KsResult<()> {
        let right = self.acc_pop(gvs)?.clone();
        if (right.value_type == INT_TYPE || right.value_type == FLOAT_TYPE) && right.value == 0 {
            return Err(KsError::runtime("Zero division error"));
        }

        let left = self.acc_pop(gvs)?.clone();

        let variable = match (left.value_type, right.value_type) {
            (INT_TYPE, INT_TYPE) => Variable::from(left.value as f64 / right.value as f64),
            (INT_TYPE, FLOAT_TYPE) | (FLOAT_TYPE, INT_TYPE) | (FLOAT_TYPE, FLOAT_TYPE) => {
                Variable::from(left.as_f64()? / right.as_f64()?)
            }
            _ => unreachable!(),
        };

        self.acc_push(gvs, variable)?;

        Ok(())
    }

    fn eq(&mut self, gvs: &mut GVS) -> KsResult<()> {
        let right = self.acc_pop(gvs)?.clone();
        let left = self.acc_pop(gvs)?.clone();

        let variable = match (left.value_type, right.value_type) {
            (INT_TYPE, INT_TYPE) => Variable::from(left.value as i64 == right.value as i64),
            (INT_TYPE, FLOAT_TYPE) | (FLOAT_TYPE, INT_TYPE) | (FLOAT_TYPE, FLOAT_TYPE) => {
                Variable::from(left.as_f64()? == right.as_f64()?)
            }
            (STRING_TYPE, STRING_TYPE) => {
                let left_string = gvs.collection_string(left.value)?;
                let right_string = gvs.collection_string(right.value)?;
                Variable::from(left_string == right_string)
            }
            _ => unreachable!(),
        };

        self.acc_push(gvs, variable)?;

        Ok(())
    }

    pub fn run(&mut self, instruction: Instruction, gvs: &mut GVS) -> KsResult<()> {
        match instruction {
            Instruction::LoadConst(constant) => self.load_const(gvs, constant),
            Instruction::LoadVar(slot) => self.load_var(gvs, slot),
            Instruction::Jump(offset) => self.jump(offset),
            Instruction::Add => self.add(gvs),
            Instruction::Minus => self.minus(gvs),
            Instruction::Mul => self.mul(gvs),
            Instruction::Div => self.div(gvs),
            Instruction::Eq => self.eq(gvs),
            _ => todo!(),
        }?;

        self.step();

        Ok(())
    }

    pub fn program_counter(&self) -> Pointer {
        self.program_counter
    }
}

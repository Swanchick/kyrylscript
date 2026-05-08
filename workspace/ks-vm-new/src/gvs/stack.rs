use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::types::{Slot, StorageId};

use super::{GVS, Variable};

#[derive(Debug)]
pub struct Stack<const SIZE: usize> {
    data: [Slot; SIZE],
    len: usize,
}

impl<const SIZE: usize> Stack<SIZE> {
    pub fn new() -> Self {
        Self {
            data: [0; SIZE],
            len: 0,
        }
    }

    pub fn from(data: [Slot; SIZE], len: usize) -> Self {
        Self { data, len }
    }

    pub fn push(&mut self, gvs: &mut GVS, variable: Variable) -> KsResult<()> {
        let storage_id = gvs.store(variable);
        gvs.storage_add_owner(storage_id)?;

        self.data[self.len] = storage_id;
        self.len = self.len.saturating_add(1);

        Ok(())
    }

    pub fn push_storage_id(&mut self, gvs: &mut GVS, storage_id: StorageId) -> KsResult<()> {
        gvs.storage_add_owner(storage_id)?;

        self.data[self.len] = storage_id;
        self.len = self.len.saturating_add(1);

        Ok(())
    }

    pub fn pop(&mut self, gvs: &mut GVS) -> KsResult<Variable> {
        let storage_id = self.data[self.len - 1];

        gvs.storage_remove_owner(storage_id)?;
        let variable = gvs.variable(storage_id)?;

        self.len = self.len.saturating_sub(1);

        Ok(variable.clone())
    }

    pub fn storage_id(&mut self, slot: Slot) -> KsResult<StorageId> {
        let slot = slot as usize;

        if slot >= self.len {
            return Err(KsError::runtime(&format!(
                "Cannot get storage_id by slot {}",
                slot
            )));
        }

        let storage_id = self.data[slot];

        Ok(storage_id)
    }

    pub fn get(&self, index: usize) -> Option<&Slot> {
        self.data.get(index)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn last<'a>(&self, gvs: &'a mut GVS) -> KsResult<&'a Variable> {
        let top = self.len.saturating_sub(1);

        let slot = self.data[top];

        let variable = gvs.variable(slot)?;
        Ok(variable)
    }

    pub fn last_mut<'a>(&mut self, gvs: &'a mut GVS) -> KsResult<&'a mut Variable> {
        let top = self.len.saturating_sub(1);

        let slot = self.data[top];

        let variable = gvs.variable_mut(slot)?;
        Ok(variable)
    }
}

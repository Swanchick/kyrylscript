use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::types::{Slot, StorageId};

use super::{GVS, Variable};

#[derive(Debug)]
pub struct Stack {
    pub data: Vec<Slot>,
}

impl From<Vec<Slot>> for Stack {
    fn from(value: Vec<Slot>) -> Self {
        Self { data: value }
    }
}

impl Stack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, gvs: &mut GVS, variable: Variable) -> KsResult<()> {
        let storage_id = gvs.store(variable);
        gvs.storage_add_owner(storage_id)?;

        self.data.push(storage_id);

        Ok(())
    }

    pub fn push_storage_id(&mut self, gvs: &mut GVS, storage_id: StorageId) -> KsResult<()> {
        gvs.storage_add_owner(storage_id)?;
        self.data.push(storage_id);

        Ok(())
    }

    pub fn pop(&mut self, gvs: &mut GVS) -> KsResult<Variable> {
        if let Some(storage_id) = self.data.pop() {
            gvs.storage_remove_owner(storage_id)?;
            let variable = gvs.variable(storage_id)?;

            Ok(variable.clone())
        } else {
            Err(KsError::runtime("No Varialbe in ACC"))
        }
    }

    pub fn storage_id(&mut self, slot: Slot) -> KsResult<StorageId> {
        if let Some(storage_id) = self.data.get(slot as usize) {
            Ok(*storage_id)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot get storage_id by slot {}",
                slot
            )))
        }
    }

    pub fn get(&self, index: usize) -> Option<&Slot> {
        self.data.get(index)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn last<'a>(&self, gvs: &'a mut GVS) -> KsResult<&'a Variable> {
        if let Some(slot) = self.data.last() {
            let variable = gvs.variable(*slot)?;
            Ok(variable)
        } else {
            Err(KsError::runtime("Cannot access last slot"))
        }
    }

    pub fn last_mut<'a>(&mut self, gvs: &'a mut GVS) -> KsResult<&'a mut Variable> {
        if let Some(slot) = self.data.last() {
            let variable = gvs.variable_mut(*slot)?;
            Ok(variable)
        } else {
            Err(KsError::runtime("Cannot access last slot"))
        }
    }

    pub fn size_pop(&mut self, size: usize) -> Vec<Slot> {
        let mut data = Vec::<Slot>::with_capacity(size);

        for _ in 0..size {
            if let Some(slot) = self.data.pop() {
                data.push(slot);
            }
        }

        data
    }
}

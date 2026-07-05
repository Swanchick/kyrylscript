use crate::types::{Slot, StorageId};

use crate::{VMError, VMResult};

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

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, gvs: &mut GVS, variable: Variable) -> VMResult<()> {
        let storage_id = gvs.store(variable);
        gvs.storage_add_owner(storage_id)?;

        self.data.push(storage_id);

        Ok(())
    }

    pub fn push_storage_id(&mut self, gvs: &mut GVS, storage_id: StorageId) -> VMResult<()> {
        gvs.storage_add_owner(storage_id)?;
        self.data.push(storage_id);

        Ok(())
    }

    pub fn pop(&mut self, gvs: &mut GVS) -> VMResult<Variable> {
        if let Some(storage_id) = self.data.pop() {
            let variable = gvs.variable(storage_id)?.clone();
            gvs.storage_remove_owner(storage_id)?;

            Ok(variable)
        } else {
            Err(VMError::from("No Varialbe in ACC"))
        }
    }

    pub fn pop_data(&mut self) -> VMResult<Slot> {
        if let Some(data) = self.data.pop() {
            Ok(data)
        } else {
            Err(VMError::from("Stack is empty"))
        }
    }

    pub fn storage_id(&mut self, slot: Slot) -> VMResult<StorageId> {
        if let Some(storage_id) = self.data.get(slot as usize) {
            Ok(*storage_id)
        } else {
            Err(VMError::from(format!(
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

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn last<'a>(&self, gvs: &'a mut GVS) -> VMResult<&'a Variable> {
        if let Some(slot) = self.data.last() {
            let variable = gvs.variable(*slot)?;
            Ok(variable)
        } else {
            Err(VMError::from("Cannot access last slot"))
        }
    }

    pub fn last_mut<'a>(&mut self, gvs: &'a mut GVS) -> VMResult<&'a mut Variable> {
        if let Some(slot) = self.data.last() {
            let variable = gvs.variable_mut(*slot)?;
            Ok(variable)
        } else {
            Err(VMError::from("Cannot access last slot"))
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

    pub fn free_last(&mut self, gvs: &mut GVS) -> VMResult<()> {
        if let Some(storage_id) = self.data.pop() {
            gvs.storage_remove_owner(storage_id)?;

            Ok(())
        } else {
            Err(VMError::from("No variable in stack"))
        }
    }
}

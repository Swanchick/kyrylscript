use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::Collection;
use crate::types::{CollectionId, Slot, StorageId};

use super::Variable;

#[derive(Debug)]
pub struct GVS {
    pub storage: Vec<Option<Variable>>,
    pub collections: Vec<Collection>,
}

impl GVS {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            collections: Vec::new(),
        }
    }

    pub fn variable(&self, storage_id: StorageId) -> KsResult<&Variable> {
        if let Some(Some(variable)) = self.storage.get(storage_id as usize) {
            Ok(variable)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot access variable {}",
                storage_id
            )))
        }
    }

    pub fn variable_mut(&mut self, storage_id: StorageId) -> KsResult<&mut Variable> {
        if let Some(Some(variable)) = self.storage.get_mut(storage_id as usize) {
            Ok(variable)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot access variable {}",
                storage_id
            )))
        }
    }

    pub fn storage_add_owner(&mut self, storage_id: StorageId) -> KsResult<()> {
        let variable = self.variable_mut(storage_id)?;
        variable.owners += 1;
        Ok(())
    }

    pub fn storage_remove_owner(&mut self, storage_id: StorageId) -> KsResult<()> {
        let variable = self.variable_mut(storage_id)?;
        variable.owners = variable.owners.saturating_sub(1);

        if variable.owners == 0 {
            todo!("Free the variable")
        }

        Ok(())
    }

    pub fn collection_string(&self, collection_id: CollectionId) -> KsResult<&str> {
        if let Some(Collection::String(string)) = self.collections.get(collection_id as usize) {
            Ok(string)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot get collection string {}",
                collection_id
            )))
        }
    }

    pub fn collection_stack(&self, collection_id: CollectionId) -> KsResult<&[Slot]> {
        if let Some(Collection::Stack(stack)) = self.collections.get(collection_id as usize) {
            Ok(stack)
        } else {
            Err(KsError::runtime(&format!(
                "Cannot get collection string {}",
                collection_id
            )))
        }
    }

    pub fn collection_store_string(&mut self, string: String) -> CollectionId {
        let collection_id = self.collections.len() as CollectionId;
        self.collections.push(Collection::String(string));

        collection_id
    }

    pub fn collection_store_stack(&mut self, stack: Vec<Slot>) -> CollectionId {
        let collection_id = self.collections.len() as CollectionId;
        self.collections.push(Collection::Stack(stack));

        collection_id
    }

    pub fn store(&mut self, variable: Variable) -> StorageId {
        let storage_id = self.storage.len() as StorageId;
        self.storage.push(Some(variable));

        storage_id
    }
}

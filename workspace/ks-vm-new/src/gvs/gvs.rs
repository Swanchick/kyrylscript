use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::Collection;
use crate::types::{CollectionId, StorageId};

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

    fn variable_mut(&mut self, storage_id: StorageId) -> KsResult<&mut Variable> {
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

    pub fn collection_store_string(&mut self, string: String) -> CollectionId {
        let collection_id = self.collections.len() as u64;
        self.collections.push(Collection::String(string));

        collection_id
    }

    pub fn store(&mut self, variable: Variable) -> StorageId {
        let storage_id = self.storage.len() as StorageId;
        self.storage.push(Some(variable));

        storage_id
    }
}

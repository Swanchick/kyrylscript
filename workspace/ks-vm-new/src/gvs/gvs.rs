use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::Collection;
use crate::gvs::variable::{
    BOOLEAN_TYPE, COLLECTION_TYPE, FLOAT_TYPE, INT_TYPE, NULL_TYPE, STRING_TYPE,
};
use crate::types::{CollectionId, Slot, StorageId};

use super::Variable;

#[derive(Debug)]
pub struct GVS {
    pub storage: Vec<Option<Variable>>,
    pub collections: Vec<Collection>,
    pub free_storage: Vec<usize>,
}

impl GVS {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            collections: Vec::new(),
            free_storage: Vec::new(),
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

    fn free_primitive(&mut self, storage_id: StorageId) -> KsResult<()> {
        let storage_id = storage_id as usize;

        self.storage[storage_id] = None;
        self.free_storage.push(storage_id);

        Ok(())
    }

    fn free(&mut self, storage_id: StorageId, value_type: u8) -> KsResult<()> {
        match value_type {
            INT_TYPE | FLOAT_TYPE | NULL_TYPE | BOOLEAN_TYPE => self.free_primitive(storage_id),
            STRING_TYPE => todo!("Free string collection"),
            COLLECTION_TYPE => todo!("Free stack collection"),
            _ => Err(KsError::runtime("Invalid variable type to free")),
        }?;

        Ok(())
    }

    pub fn storage_remove_owner(&mut self, storage_id: StorageId) -> KsResult<()> {
        let (owners, value_type) = {
            let variable = self.variable_mut(storage_id)?;
            variable.owners = variable.owners.saturating_sub(1);
            (variable.owners, variable.value_type)
        };

        if owners == 0 {
            self.free(storage_id, value_type)?;
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
        if let Some(storage_id) = self.free_storage.pop() {
            self.storage[storage_id] = Some(variable);

            storage_id as StorageId
        } else {
            let storage_id = self.storage.len() as StorageId;
            self.storage.push(Some(variable));

            storage_id
        }
    }

    pub fn variable_iter<COLLECTION, PRIMITIVE>(
        &mut self,
        storage_id: StorageId,
        mut collection_func: COLLECTION,
        mut primitive_func: PRIMITIVE,
    ) -> KsResult<()>
    where
        COLLECTION: FnMut(&mut Self, StorageId) -> KsResult<()>,
        PRIMITIVE: FnMut(&mut Self, StorageId) -> KsResult<()>,
    {
        // let mut collections = vec![storage_id];

        // while let Some(storage_id) = splits {}

        Ok(())
    }
}

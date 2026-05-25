use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::Collection;
use crate::types::{CollectionId, Slot, StorageId};

use super::Variable;
use super::frame::Frame;
use super::variable::{
    BOOLEAN_TYPE, COLLECTION_TYPE, FLOAT_TYPE, INT_TYPE, NULL_TYPE, STRING_TYPE,
};

#[derive(Debug)]
pub struct GVS {
    pub storage: Vec<Option<Variable>>,
    pub collections: Vec<Collection>,
    pub free_storage: Vec<usize>,
    pub free_collection: Vec<usize>,
}

impl Default for GVS {
    fn default() -> Self {
        Self::new()
    }
}

impl GVS {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            collections: Vec::new(),
            free_storage: Vec::new(),
            free_collection: Vec::new(),
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

    fn free_primitive(&mut self, storage_id: StorageId) {
        let storage_id = storage_id as usize;
        self.storage[storage_id] = None;
        self.free_storage.push(storage_id);
    }

    fn free_string(&mut self, storage_id: StorageId) -> KsResult<()> {
        let collection_id = {
            let variable = self.variable(storage_id)?;
            variable.value as usize
        };

        self.collections[collection_id] = Collection::Free;
        self.free_collection.push(storage_id as usize);

        Ok(())
    }

    fn free_collection(&mut self, collection_id: CollectionId) {
        let collection_id = collection_id as usize;
        self.collections[collection_id] = Collection::Free;
        self.free_collection.push(collection_id);
    }

    fn free_stack(&mut self, storage_id: StorageId) -> KsResult<()> {
        self.collection_iter(
            storage_id,
            |gvs, current_storage_id| {
                gvs.storage_remove_owner(current_storage_id)?;

                Ok(())
            },
            |gvs, current_storage_id| {
                let (collection_id, owners) = {
                    let variable = gvs.variable_mut(current_storage_id)?;
                    variable.owners = variable.owners.saturating_sub(1);
                    (variable.value, variable.owners)
                };

                if current_storage_id == storage_id {
                    gvs.free_collection(collection_id);
                    return Ok(());
                }

                if owners == 0 {
                    gvs.free_collection(collection_id);
                    gvs.free_primitive(current_storage_id);
                }

                Ok(())
            },
        )?;

        Ok(())
    }

    fn free(&mut self, storage_id: StorageId, value_type: u8) -> KsResult<()> {
        match value_type {
            INT_TYPE | FLOAT_TYPE | NULL_TYPE | BOOLEAN_TYPE => Ok(()),
            STRING_TYPE => self.free_string(storage_id),
            COLLECTION_TYPE => self.free_stack(storage_id),
            _ => Err(KsError::runtime("Invalid variable type to free")),
        }?;

        self.free_primitive(storage_id);

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
                "Cannot get collection stack {}",
                collection_id
            )))
        }
    }

    pub fn collection_store_string(&mut self, string: String) -> CollectionId {
        if let Some(collection_id) = self.free_collection.pop() {
            self.collections[collection_id] = Collection::String(string);
            collection_id as CollectionId
        } else {
            let collection_id = self.collections.len() as CollectionId;
            self.collections.push(Collection::String(string));

            collection_id
        }
    }

    pub fn collection_store_stack(&mut self, stack: Vec<Slot>) -> CollectionId {
        if let Some(collection_id) = self.free_collection.pop() {
            self.collections[collection_id] = Collection::Stack(stack);

            collection_id as CollectionId
        } else {
            let collection_id = self.collections.len() as CollectionId;
            self.collections.push(Collection::Stack(stack));

            collection_id
        }
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

    fn collection_iter<VARIABLE, STACK>(
        &mut self,
        storage_id: StorageId,
        mut variable_func: VARIABLE,
        mut stack_func: STACK,
    ) -> KsResult<()>
    where
        VARIABLE: FnMut(&mut Self, StorageId) -> KsResult<()>,
        STACK: FnMut(&mut Self, StorageId) -> KsResult<()>,
    {
        let collection_id = {
            let variable = self.variable(storage_id)?;

            if variable.value_type != COLLECTION_TYPE {
                return Err(KsError::runtime(
                    "Cannot iterate over non collection variable",
                ));
            }

            variable.value
        };

        let mut collections = vec![Frame::new(storage_id, collection_id)];

        while let Some(mut frame) = collections.pop() {
            let collection = self.collection_stack(frame.collection_id)?;

            if let Some(storage_id) = collection.get(frame.index) {
                frame.index += 1;

                collections.push(frame);

                let variable = self.variable(*storage_id)?;
                if variable.value_type == COLLECTION_TYPE {
                    collections.push(Frame::new(*storage_id, variable.value));
                    continue;
                }

                variable_func(self, *storage_id)?;
            } else {
                stack_func(self, frame.storage_id)?;
            }
        }

        Ok(())
    }
}

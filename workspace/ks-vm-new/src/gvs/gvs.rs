use crate::Collection;
use crate::types::{CollectionId, StorageId};

use super::Variable;

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

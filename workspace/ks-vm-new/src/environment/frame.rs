use crate::types::{CollectionId, StorageId};

pub struct Frame {
    pub storage_id: StorageId,
    pub collection_id: CollectionId,
    pub index: usize,
}

impl Frame {
    pub fn new(storage_id: StorageId, collection_id: CollectionId) -> Self {
        Self {
            storage_id,
            collection_id,
            index: 0,
        }
    }
}

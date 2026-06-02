use super::types::{Pointer, StorageId};

#[derive(Debug)]
pub struct CallStack {
    pub return_pointer: Pointer,
    pub collection_id: Pointer,
    pub storage_id: StorageId,
}

impl CallStack {
    pub fn new(return_pointer: Pointer, collection_id: Pointer, storage_id: StorageId) -> Self {
        Self {
            return_pointer,
            collection_id,
            storage_id,
        }
    }
}

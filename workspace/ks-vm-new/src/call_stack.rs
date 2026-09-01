use super::types::{Pointer, StorageId};

#[derive(Debug)]
pub struct CallStack {
    pub return_pointer: Pointer,
    pub stack_pointer: Pointer,
    pub storage_id: StorageId,
}

impl CallStack {
    pub fn new(return_pointer: Pointer, stack_pointer: Pointer, storage_id: StorageId) -> Self {
        Self {
            return_pointer,
            stack_pointer,
            storage_id,
        }
    }
}

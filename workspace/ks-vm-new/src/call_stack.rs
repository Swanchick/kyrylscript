use super::types::Pointer;

#[derive(Debug)]
pub struct CallStack {
    pub return_pointer: Pointer,
    pub collection_id: Pointer,
}

impl CallStack {
    pub fn new(return_pointer: Pointer, collection_id: Pointer) -> Self {
        Self {
            return_pointer,
            collection_id,
        }
    }
}

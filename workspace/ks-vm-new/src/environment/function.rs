use ks_global::utils::{ks_error::KsError, ks_result::KsResult};

const EMPTY_COLLECTION: u32 = 0xFFFFFFFF;

pub struct Function {
    pub collection_id: Option<u32>,
    pub pointer: u32,
}

impl From<u32> for Function {
    fn from(pointer: u32) -> Self {
        Function {
            collection_id: None,
            pointer,
        }
    }
}

impl From<u64> for Function {
    fn from(value: u64) -> Self {
        let pointer = value as u32;
        let collection_id = (value >> 32) as u32;

        let collection_id = if collection_id == EMPTY_COLLECTION {
            None
        } else {
            Some(collection_id)
        };

        Function {
            collection_id,
            pointer,
        }
    }
}

impl Function {
    pub fn new(pointer: u32, collection_id: u32) -> Self {
        Self {
            collection_id: Some(collection_id),
            pointer,
        }
    }

    pub fn as_u64(self) -> u64 {
        let pointer = self.pointer as u64;
        let collection_id = self.collection_id.unwrap_or(EMPTY_COLLECTION) as u64;

        collection_id << 32 | pointer
    }

    pub fn collection_id(&self) -> KsResult<usize> {
        if let Some(collection_id) = self.collection_id {
            Ok(collection_id as usize)
        } else {
            Err(KsError::runtime(
                "Function does not have any captured variables",
            ))
        }
    }
}

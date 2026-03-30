use crate::types::{Stack, StorageId};

use super::Variable;

pub struct GVS {
    pub storage: Vec<Option<Variable>>,
    pub stacks: Vec<Stack>,
}

impl GVS {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            stacks: Vec::new(),
        }
    }

    pub fn store(&mut self, variable: Variable) -> StorageId {
        let storage_id = self.storage.len() as StorageId;
        self.storage.push(Some(variable));

        storage_id
    }
}

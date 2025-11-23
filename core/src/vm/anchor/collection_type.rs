use std::collections::HashMap;

use crate::global::utils::{ks_error::KsError, ks_result::KsResult};

pub enum CollectionType {
    List(Vec<u64>),
    Module(HashMap<String, u64>),
}

impl CollectionType {
    pub fn add_to_list(&mut self, reference: u64) -> KsResult<()> {
        match self {
            CollectionType::List(references) => {
                references.push(reference);
                Ok(())
            }
            _ => Err(KsError::runtime(
                "Cannot clone add reference to the module!",
            )),
        }
    }
}

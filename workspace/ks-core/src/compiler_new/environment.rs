use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use std::collections::HashMap;

use super::types::CollectionId;

use super::collection::Collection;
use super::slot::Slot;
use super::types::{Pointer, VariableId};

pub struct Environment {
    functions: HashMap<String, Pointer>,
    variables: HashMap<String, Slot>,
    collections: Vec<Collection>,
    temp_collection: Option<CollectionId>,
    current: usize,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            functions: HashMap::new(),
            variables: HashMap::new(),
            collections: Vec::new(),
            temp_collection: None,
            current: 0,
        }
    }

    pub fn functions(self) -> HashMap<String, Pointer> {
        self.functions
    }

    pub fn temp_collection(&mut self) -> Option<CollectionId> {
        self.temp_collection.take()
    }

    pub fn register_collection(&mut self, collection: Collection) -> CollectionId {
        let collection_id = self.collections.len();
        self.collections.push(collection);
        collection_id
    }

    pub fn set_temp_collection(&mut self, collection_id: CollectionId) {
        self.temp_collection = Some(collection_id);

        println!("{:?}", self.temp_collection);
    }

    pub fn define_variable(&mut self, name: String) -> KsResult<VariableId> {
        let variable_id = self.current;

        let temp_collection = self.temp_collection();
        println!("{:?}", temp_collection);

        let slot = if let Some(collection_id) = temp_collection {
            Slot::Collection {
                variable_id,
                collection_id,
            }
        } else {
            Slot::Variable(variable_id)
        };

        self.variables.insert(name, slot);
        self.current += 1;

        Ok(variable_id)
    }

    pub fn define_function(&mut self, name: &str, pointer: Pointer) {
        self.functions.insert(name.to_string(), pointer);
    }

    pub fn variable_id(&self, name: &str) -> KsResult<VariableId> {
        if let Some(variable_id) = self.variables.get(name) {
            Ok(*variable_id.variable_id())
        } else {
            Err(KsError::parse(&format!(
                "Cannot find variable by this name: {}",
                name
            )))
        }
    }

    pub fn slot(&self, name: &str) -> KsResult<&Slot> {
        if let Some(slot) = self.variables.get(name) {
            Ok(slot)
        } else {
            Err(KsError::parse(&format!(
                "Cannot find slot by this name: {}",
                name
            )))
        }
    }

    pub fn collection(&self, collection_id: CollectionId) -> KsResult<&Collection> {
        if let Some(collection) = self.collections.get(collection_id) {
            Ok(collection)
        } else {
            Err(KsError::parse("Cannot find collection"))
        }
    }
}

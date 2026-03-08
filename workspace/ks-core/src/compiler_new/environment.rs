use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use std::collections::HashMap;

use super::collection::Collection;
use super::function::Function;
use super::slot::Slot;
use super::types::{CollectionId, Depth, Pointer, VariableId};

pub struct Environment {
    functions: HashMap<String, Function>,
    variables: Vec<HashMap<String, Slot>>,
    collections: Vec<Collection>,
    temp_collection: Option<CollectionId>,
    current: usize,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            functions: HashMap::new(),
            variables: Vec::new(),
            collections: Vec::new(),
            temp_collection: None,
            current: 0,
        }
    }

    fn depth(&self) -> Depth {
        self.variables.len().saturating_sub(1)
    }

    fn current(&self) -> VariableId {
        self.variables.iter().map(|scope| scope.len()).sum()
    }

    pub fn functions(self) -> HashMap<String, Pointer> {
        self.functions
            .into_iter()
            .map(|(name, function)| (name, function.pointer))
            .collect()
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

    pub fn enter(&mut self) {
        self.variables.push(HashMap::new());
    }

    pub fn exit(&mut self) -> KsResult<HashMap<String, Slot>> {
        if let Some(scope) = self.variables.pop() {
            Ok(scope)
        } else {
            Err(KsError::parse("No scope to exit"))
        }
    }

    pub fn current_scope_mut(&mut self) -> KsResult<&mut HashMap<String, Slot>> {
        if let Some(scope) = self.variables.last_mut() {
            Ok(scope)
        } else {
            Err(KsError::parse("Cannot get last scope"))
        }
    }

    pub fn define_variable(&mut self, name: String) -> KsResult<VariableId> {
        let variable_id = self.current();

        let temp_collection = self.temp_collection();

        let slot = if let Some(collection_id) = temp_collection {
            Slot::Collection {
                variable_id,
                collection_id,
            }
        } else {
            Slot::Variable(variable_id)
        };

        let current_scope = self.current_scope_mut()?;
        current_scope.insert(name, slot);

        Ok(variable_id)
    }

    pub fn define_function(&mut self, name: &str, pointer: Pointer) {
        let function = Function::new(pointer, self.depth());

        self.functions.insert(name.to_string(), function);
    }

    pub fn slot(&self, name: &str) -> KsResult<&Slot> {
        for scope in &self.variables {
            if let Some(slot) = scope.get(name) {
                return Ok(slot);
            }
        }

        Err(KsError::parse(&format!(
            "Cannot find variable by this name: {}",
            name
        )))
    }

    pub fn collection(&self, collection_id: CollectionId) -> KsResult<&Collection> {
        if let Some(collection) = self.collections.get(collection_id) {
            Ok(collection)
        } else {
            Err(KsError::parse("Cannot find collection"))
        }
    }
}

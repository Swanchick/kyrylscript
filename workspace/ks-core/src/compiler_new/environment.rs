use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use std::collections::HashMap;

use crate::compiler_new::types::NativeId;

use super::collection::Collection;
use super::slot::Slot;
use super::types::{CollectionId, Pointer, VariableId};

pub struct Environment {
    functions: HashMap<String, Pointer>,
    variables: Vec<Vec<HashMap<String, Slot>>>,
    native_function: HashMap<String, NativeId>,
    collections: Vec<Collection>,
    temp_collection: Option<CollectionId>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            functions: HashMap::new(),
            variables: vec![Vec::new()],
            native_function: HashMap::new(),
            collections: Vec::new(),
            temp_collection: None,
        }
    }

    pub fn register_native(&mut self, name: String, native_id: NativeId) {
        self.native_function.insert(name, native_id);
    }

    pub fn native_function(&mut self, name: &String) -> Option<&NativeId> {
        self.native_function.get(name)
    }

    fn function_variables_len(&self, variables: &[HashMap<String, Slot>]) -> usize {
        variables.iter().map(|scope| scope.len()).sum()
    }

    pub fn clear_temp_collection(&mut self) {
        self.temp_collection = None;
    }

    pub fn current(&self) -> KsResult<VariableId> {
        let variables = self.last_function()?;

        Ok(self.function_variables_len(variables) as u64)
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
    }

    fn last_function(&self) -> KsResult<&[HashMap<String, Slot>]> {
        if let Some(function_scope) = self.variables.last() {
            Ok(function_scope)
        } else {
            Err(KsError::parse("No function scope declared"))
        }
    }

    fn last_function_mut(&mut self) -> KsResult<&mut Vec<HashMap<String, Slot>>> {
        if let Some(function_scope) = self.variables.last_mut() {
            Ok(function_scope)
        } else {
            Err(KsError::parse("No function scope declared"))
        }
    }

    pub fn enter_function(&mut self) -> KsResult<()> {
        self.variables.push(Vec::new());
        self.enter()?;

        Ok(())
    }

    pub fn exit_function(&mut self) -> KsResult<usize> {
        if let Some(function_scope) = self.variables.pop() {
            let function_variables_len = self.function_variables_len(&function_scope);

            Ok(function_variables_len)
        } else {
            Err(KsError::parse("No function environment to exit"))
        }
    }

    pub fn enter(&mut self) -> KsResult<()> {
        let variables = self.last_function_mut()?;
        variables.push(HashMap::new());

        Ok(())
    }

    pub fn exit(&mut self) -> KsResult<HashMap<String, Slot>> {
        let variables = self.last_function_mut()?;

        if let Some(scope) = variables.pop() {
            Ok(scope)
        } else {
            Err(KsError::parse("No scope to exit"))
        }
    }

    fn current_scope_mut(&mut self) -> KsResult<&mut HashMap<String, Slot>> {
        let variables = self.last_function_mut()?;

        if let Some(scope) = variables.last_mut() {
            Ok(scope)
        } else {
            Err(KsError::parse("Cannot get last scope"))
        }
    }

    pub fn define_variable(&mut self, name: String) -> KsResult<VariableId> {
        let variable_id = self.current()?;

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
        self.functions.insert(name.to_string(), pointer);
    }

    pub fn slot(&self, name: &str) -> KsResult<&Slot> {
        let variables = self.last_function()?;

        for scope in variables {
            if let Some(slot) = scope.get(name) {
                return Ok(slot);
            }
        }

        Err(KsError::parse(&format!(
            "Cannot find variable by this name: {}",
            name
        )))
    }

    pub fn variable_id(&self, name: &str) -> KsResult<VariableId> {
        let slot = self.slot(name)?;

        Ok(*slot.variable_id())
    }

    pub fn collection(&self, collection_id: CollectionId) -> KsResult<&Collection> {
        if let Some(collection) = self.collections.get(collection_id) {
            Ok(collection)
        } else {
            Err(KsError::parse("Cannot find collection"))
        }
    }
}

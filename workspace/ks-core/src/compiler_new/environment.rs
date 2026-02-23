use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use std::collections::HashMap;

use super::collection::Collection;
use super::types::{Pointer, VariableId};

pub struct Environment {
    functions: HashMap<String, Pointer>,
    variables: HashMap<String, VariableId>,
    modules: HashMap<VariableId, Collection>,
    temporary_module: Vec<Collection>,
    current: usize,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            functions: HashMap::new(),
            variables: HashMap::new(),
            modules: HashMap::new(),
            temporary_module: Vec::new(),
            current: 0,
        }
    }

    pub fn functions(self) -> HashMap<String, Pointer> {
        self.functions
    }

    pub fn define_variable(&mut self, name: String) -> KsResult<VariableId> {
        let current_count = self.current;

        self.variables.insert(name, current_count);
        self.current += 1;

        Ok(current_count)
    }

    pub fn create_module(&mut self) -> KsResult<()> {
        if let Some(module) = self.temporary_module.last() {
            let variable_id = module.len()?;
            self.temporary_module.push(Collection::Module {
                variable_id: Some(variable_id),
                fields: HashMap::new(),
            });
        } else {
            self.temporary_module.push(Collection::Module {
                variable_id: None,
                fields: HashMap::new(),
            });
        }

        Ok(())
    }

    fn module_pop(&mut self) -> KsResult<Collection> {
        if let Some(module) = self.temporary_module.pop() {
            Ok(module)
        } else {
            Err(KsError::parse("Cannot get last temporary module"))
        }
    }

    fn module_last_mut(&mut self) -> KsResult<&mut Collection> {
        if let Some(module) = self.temporary_module.last_mut() {
            Ok(module)
        } else {
            Err(KsError::parse("Cannot get last temporary module"))
        }
    }

    pub fn insert_field(&mut self, name: String) -> KsResult<()> {
        let last_module = self.module_last_mut()?;
        last_module.insert_field(name)?;
        Ok(())
    }

    pub fn insert_module(&mut self, name: String) -> KsResult<()> {
        let module = self.module_pop()?;
        let last_module = self.module_last_mut()?;
        last_module.insert_module(name, module)?;
        Ok(())
    }

    pub fn temporary_modules_len(&self) -> usize {
        self.temporary_module.len()
    }

    pub fn define_module_if_created(&mut self, variable_id: VariableId) -> KsResult<()> {
        if let Some(module) = self.temporary_module.pop() {
            self.modules.insert(variable_id, module);
        }
        Ok(())
    }

    pub fn define_function(&mut self, name: &str, pointer: Pointer) {
        self.functions.insert(name.to_string(), pointer);
    }

    pub fn variable_id(&self, name: &str) -> KsResult<VariableId> {
        if let Some(variable_id) = self.variables.get(name) {
            Ok(*variable_id)
        } else {
            Err(KsError::parse(&format!(
                "Did not find variable by this name: {}",
                name
            )))
        }
    }
}

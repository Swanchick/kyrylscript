use ks_global::utils::{ks_error::KsError, ks_result::KsResult};
use std::collections::HashMap;

use crate::compiler_new::types::{Pointer, VariableId};

pub struct Environment {
    functions: HashMap<String, Pointer>,
    variables: HashMap<String, usize>,
    current: usize,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            functions: HashMap::new(),
            variables: HashMap::new(),
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

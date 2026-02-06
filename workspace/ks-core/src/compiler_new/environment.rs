use ks_global::utils::{ks_error::KsError, ks_result::KsResult};
use std::collections::HashMap;

use crate::compiler_new::types::VariableId;

pub struct Environment {
    variables: Vec<HashMap<String, usize>>,
    counters: Vec<usize>,
    current: usize,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: vec![HashMap::new()],
            counters: Vec::new(),
            current: 0,
        }
    }

    pub fn define_variable(&mut self, name: &str) -> KsResult<VariableId> {
        let current_count = self.current;

        let scope = self.current_scope_mut()?;
        scope.insert(name.to_string(), current_count);
        self.current += 1;

        Ok(current_count)
    }

    pub fn enter(&mut self) {
        self.variables.push(HashMap::new());
    }

    pub fn exit(&mut self) {
        self.variables.push(HashMap::new());
    }

    fn current_scope_mut(&mut self) -> KsResult<&mut HashMap<String, usize>> {
        if let Some(scope) = self.variables.last_mut() {
            Ok(scope)
        } else {
            Err(KsError::parse("Scope not found"))
        }
    }
}

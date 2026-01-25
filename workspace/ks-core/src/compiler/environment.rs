use ks_global::utils::{ks_error::KsError, ks_result::KsResult};
use ks_vm::function::Function;
use std::collections::HashMap;

pub struct Environment {
    functions: Vec<Function>,
    variables: Vec<HashMap<String, usize>>,
    counters: Vec<usize>,
    current: usize,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            functions: Vec::new(),
            variables: vec![HashMap::new()],
            counters: Vec::new(),
            current: 0,
        }
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn create_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    pub fn create_variable(&mut self, name: &str) -> KsResult<()> {
        let current_count = self.current;

        let scope = self.current_scope_mut()?;
        scope.insert(name.to_string(), current_count);
        self.current += 1;

        Ok(())
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

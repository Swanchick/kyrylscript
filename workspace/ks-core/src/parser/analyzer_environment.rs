use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use super::data_type::DataType;

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyzerEnvironment {
    parent: Option<Rc<RefCell<AnalyzerEnvironment>>>,
    variables: HashMap<String, DataType>,
}

impl AnalyzerEnvironment {
    pub fn new() -> AnalyzerEnvironment {
        AnalyzerEnvironment {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn with_parent(parent: Rc<RefCell<AnalyzerEnvironment>>) -> AnalyzerEnvironment {
        AnalyzerEnvironment {
            parent: Some(parent),
            variables: HashMap::new(),
        }
    }

    pub fn get_variables(&self) -> &HashMap<String, DataType> {
        &self.variables
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<AnalyzerEnvironment>>> {
        match &self.parent {
            Some(parent) => Some(parent.clone()),
            None => None,
        }
    }

    pub fn get_variable_type(&self, name: &str) -> KsResult<DataType> {
        if let Some(data_type) = self.variables.get(name) {
            Ok(data_type.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_variable_type(name)
        } else {
            Err(KsError::parse(&format!("Variable {} not found", name)))
        }
    }

    pub fn add(&mut self, name: String, data_type: DataType) {
        self.variables.insert(name, data_type);
    }
}

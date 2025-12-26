use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

use super::data_type::DataType;

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyzerEnviroment {
    parent: Option<Rc<RefCell<AnalyzerEnviroment>>>,
    variables: HashMap<String, DataType>,
}

impl AnalyzerEnviroment {
    pub fn new() -> AnalyzerEnviroment {
        AnalyzerEnviroment {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn with_parent(parent: Rc<RefCell<AnalyzerEnviroment>>) -> AnalyzerEnviroment {
        AnalyzerEnviroment {
            parent: Some(parent),
            variables: HashMap::new(),
        }
    }

    pub fn get_variables(&self) -> &HashMap<String, DataType> {
        &self.variables
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<AnalyzerEnviroment>>> {
        match &self.parent {
            Some(parent) => Some(parent.clone()),
            None => None,
        }
    }

    pub fn get_variable_type(&self, name: &str) -> io::Result<DataType> {
        if let Some(data_type) = self.variables.get(name) {
            Ok(data_type.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_variable_type(name)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Variable {} not found", name),
            ))
        }
    }

    pub fn add(&mut self, name: String, data_type: DataType) {
        self.variables.insert(name, data_type);
    }
}

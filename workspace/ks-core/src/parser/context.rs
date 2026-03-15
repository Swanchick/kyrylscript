use super::data_type::DataType;

#[derive(Debug, Clone)]
pub struct Context {
    pub captured_variables: Vec<String>,
    pub variables: Vec<String>,
    pub return_type: DataType,
}

impl Context {
    pub fn new(return_type: DataType) -> Self {
        Self {
            captured_variables: Vec::new(),
            variables: Vec::new(),
            return_type,
        }
    }

    pub fn process() -> Self {
        Self {
            captured_variables: Vec::new(),
            variables: Vec::new(),
            return_type: DataType::void(),
        }
    }
}

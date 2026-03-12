use super::data_type::DataType;

#[derive(Debug, Clone)]
pub struct Context {
    pub variables: Vec<String>,
    pub return_type: DataType,
}

impl Context {
    pub fn new(return_type: DataType) -> Self {
        Self {
            variables: Vec::new(),
            return_type,
        }
    }

    pub fn process() -> Self {
        Self {
            variables: Vec::new(),
            return_type: DataType::void(),
        }
    }
}

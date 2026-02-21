use std::collections::BTreeMap;

use crate::parser::parameter::Parameter;

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Void(Option<Box<DataType>>),
    List(Box<DataType>),
    Tuple(Vec<DataType>),
    Module(BTreeMap<String, DataType>),
    RustFunction {
        return_type: Box<DataType>,
    },
    Function {
        parameters: Vec<DataType>,
        return_type: Box<DataType>,
    },
}

impl DataType {
    pub fn from_parameters(parameters: &[Parameter]) -> Vec<DataType> {
        let mut out: Vec<DataType> = Vec::new();

        for parameter in parameters {
            out.push(parameter.data_type.clone());
        }

        out
    }

    pub fn void() -> DataType {
        DataType::Void(None)
    }

    pub fn is_void(data_type: &DataType) -> bool {
        matches!(data_type, DataType::Void(_))
    }
}

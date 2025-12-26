use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Void(Option<Box<DataType>>),
    List(Box<DataType>),
    Tuple(Vec<DataType>),
    Module(HashMap<String, DataType>),
    RustFunction {
        return_type: Box<DataType>,
    },
    Function {
        parameters: Vec<DataType>,
        return_type: Box<DataType>,
    },
}

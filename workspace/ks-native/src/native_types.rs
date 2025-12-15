use std::collections::HashMap;

use super::native_function::NativeFunction;

#[derive(Debug, Clone)]
pub enum NativeType {
    Function(NativeFunction),
    Int(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Tuple(Vec<NativeType>),
    Module(HashMap<String, NativeType>),
}

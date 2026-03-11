use crate::environment::Reference;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    List(Vec<Reference>),
    Tuple(Vec<Reference>),
    Function(String),
    NativeFunction(String),
    Module(HashMap<String, Reference>),
}

use std::io;

use crate::vm::variable::Variable;
use crate::global::data_type::DataType;


#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub function: fn(args: Vec<Variable>) -> io::Result<Variable>,
    pub return_type: DataType
}

impl NativeFunction {
    pub fn from(function: fn(args: Vec<Variable>) -> io::Result<Variable>, return_type: DataType) -> NativeFunction {
        NativeFunction { 
            function: function, 
            return_type: return_type 
        }
    }
    
    pub fn process(function: fn(args: Vec<Variable>) -> io::Result<Variable>) -> NativeFunction {
        NativeFunction {
            function,
            return_type: DataType::void()
        }
    }
}

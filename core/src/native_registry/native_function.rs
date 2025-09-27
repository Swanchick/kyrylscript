use crate::global::utils::ks_result::KsResult;
use crate::vm::environment::Environment;
use crate::vm::variable::Variable;
use crate::global::data_type::DataType;


#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub function: fn(environment: &mut Environment, args: Vec<Variable>) -> KsResult<Variable>,
    pub return_type: DataType
}

impl NativeFunction {
    pub fn from(function: fn(environment: &mut Environment, args: Vec<Variable>) -> KsResult<Variable>, return_type: DataType) -> NativeFunction {
        NativeFunction { 
            function: function, 
            return_type: return_type 
        }
    }
    
    pub fn process(function: fn(environment: &mut Environment, args: Vec<Variable>) -> KsResult<Variable>) -> NativeFunction {
        NativeFunction {
            function,
            return_type: DataType::void()
        }
    }
}

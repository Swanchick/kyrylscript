use ks_global::utils::ks_result::KsResult;

use crate::environment::Environment;
use crate::variable::Variable;

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub function: fn(environment: &mut Environment, args: Vec<Variable>) -> KsResult<Variable>,
}

impl NativeFunction {
    pub fn from(
        function: fn(environment: &mut Environment, args: Vec<Variable>) -> KsResult<Variable>,
    ) -> NativeFunction {
        NativeFunction { function }
    }
}

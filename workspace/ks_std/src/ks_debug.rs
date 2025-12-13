use core::global::utils::ks_result::KsResult;
use core::vm::environment::Environment;

use core::vm::variable::Variable;

pub fn ks_debug(environment: &mut Environment, _: Vec<Variable>) -> KsResult<Variable> {
    environment.debug();

    Ok(Variable::null())
}

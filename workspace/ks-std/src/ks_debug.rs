use ks_global::utils::ks_result::KsResult;
use ks_vm::environment::Environment;
use ks_vm::variable::Variable;

pub fn ks_debug(environment: &mut Environment, _: Vec<Variable>) -> KsResult<Variable> {
    environment.debug();

    Ok(Variable::null())
}

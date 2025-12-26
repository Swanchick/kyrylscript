use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use ks_vm::environment::Environment;
use ks_vm::variable::Variable;
use ks_vm::variable::value::Value;

pub fn ks_ref(_: &mut Environment, args: Vec<Variable>) -> KsResult<Variable> {
    if args.len() > 1 {
        return Err(KsError::native("Too many arguments!"));
    }

    let variable = &args[0];
    let reference = variable.reference()?;

    Ok(Variable::empty(Value::Integer(reference as i32)))
}

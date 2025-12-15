use core::global::utils::ks_error::KsError;
use core::global::utils::ks_result::KsResult;
use core::vm::environment::Environment;
use core::vm::value::Value;
use core::vm::variable::Variable;

pub fn ks_ref(_: &mut Environment, args: Vec<Variable>) -> KsResult<Variable> {
    if args.len() > 1 {
        return Err(KsError::native("Too many arguments!"));
    }

    let variable = &args[0];
    let reference = variable.reference()?;

    Ok(Variable::empty(Value::Integer(reference as i32)))
}

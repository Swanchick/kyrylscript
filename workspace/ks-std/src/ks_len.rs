use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use ks_vm::environment::Environment;
use ks_vm::variable::Variable;
use ks_vm::variable::value::Value;

pub fn ks_len(_: &mut Environment, args: Vec<Variable>) -> KsResult<Variable> {
    if args.len() > 1 {
        return Err(KsError::runtime("Too many arguments!"));
    }

    let variable = &args[0];
    match variable.value() {
        Value::List(references) | Value::Tuple(references) => {
            let variable = Variable::empty(Value::Integer(references.len() as i32));

            Ok(variable)
        }
        Value::String(string) => {
            let variable = Variable::empty(Value::Integer(string.len() as i32));

            Ok(variable)
        }

        _ => Err(KsError::native("Invalid type argument!")),
    }
}

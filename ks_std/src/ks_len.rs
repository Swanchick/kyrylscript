use core::global::utils::ks_error::KsError;

use core::global::utils::ks_result::KsResult;
use core::vm::value::Value;
use core::vm::variable::Variable;
use core::vm::environment::Environment;



pub fn ks_len(environment: &mut Environment, args: Vec<Variable>) -> KsResult<Variable> {
    if args.len() > 1 {
        return Err(KsError::runtime("Too many arguments!"));
    }

    let variable = &args[0];
    match variable.value() {
        Value::List(references)
        | Value::Tuple(references) => {
            let variable = Variable::empty(
                Value::Integer(references.len() as i32), 
                environment.depth()
            );

            Ok(variable)
        },
        Value::String(string) => {
            let variable = Variable::empty(
                Value::Integer(string.len() as i32), 
                environment.depth()
            );

            Ok(variable)
        }

        _ => Err(KsError::native("Invalid type argument!"))
    }
}
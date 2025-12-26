use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use ks_vm::environment::Environment;
use ks_vm::native::native_helper::NativeHelper;
use ks_vm::variable::Variable;
use ks_vm::variable::value::Value;

pub fn ks_range(environment: &mut Environment, args: Vec<Variable>) -> KsResult<Variable> {
    if args.len() > 1 {
        return Err(KsError::native("Too many arguments!"));
    }

    if let Value::Integer(int) = args[0].value() {
        let mut variables: Vec<Variable> = Vec::new();

        for i in 0..(*int as usize) {
            let variable = Variable::empty(Value::Integer(i as i32));

            variables.push(variable);
        }

        let mut helper = NativeHelper::from(environment);

        let references = helper.create_collections(variables)?;

        Ok(Variable::empty(Value::List(references)))
    } else {
        Err(KsError::native("Wrong type. Was expected int!"))
    }
}

use core::global::utils::ks_error::KsError;
use core::native_registry::native_helper::NativeHelper;
use core::vm::environment::Environment;

use core::global::utils::ks_result::KsResult;
use core::vm::value::Value;
use core::vm::variable::Variable;


pub fn ks_range(environment: &mut Environment, args: Vec<Variable>) -> KsResult<Variable> { 
    if args.len() > 1 {
        return Err(KsError::native("Too many arguments!"));
    }
    
    if let Value::Integer(int) = args[0].value() {
        let mut variables: Vec<Variable> = Vec::new();
        
        for i in 0..(*int as usize) {
            let variable = Variable::empty(
                Value::Integer(i as i32), 
            );

            variables.push(variable);
        }

        let mut helper = NativeHelper::from(environment);

        let references = helper.create_collections(variables)?;

        Ok(Variable::empty(
            Value::List(references), 
        ))
    } else {
        Err(KsError::native("Wrong type. Was expected int!"))
    }
}
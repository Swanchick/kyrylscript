use std::collections::HashMap;

use ks_global::utils::ks_result::KsResult;
use ks_vm::environment::{Environment, Reference};
use ks_vm::variable::Variable;
use ks_vm::variable::value::Value;

fn collection_to_string(
    environment: &mut Environment,
    references: &[Reference],
    mut buffer: String,
) -> KsResult<String> {
    let references_len = references.len();

    for (i, reference) in references.iter().enumerate() {
        let variable = environment.variable(reference)?;
        let variable_string = value_to_string(environment, variable.clone())?;
        buffer.push_str(variable_string.as_str());

        if i != references_len - 1 {
            buffer.push_str(", ");
        }
    }

    Ok(buffer)
}

fn module_to_string(
    environment: &mut Environment,
    module: &HashMap<String, Reference>,
    mut buffer: String,
) -> KsResult<String> {
    let module_len = module.len();

    for (i, (name, reference)) in module.iter().enumerate() {
        let variable = environment.variable(reference)?;
        let variable_string = value_to_string(environment, variable.clone())?;
        buffer.push_str(name);
        buffer.push_str(": ");
        buffer.push_str(variable_string.as_str());

        if i != module_len - 1 {
            buffer.push_str(", ");
        }
    }

    Ok(buffer)
}

fn value_to_string(environment: &mut Environment, variable: Variable) -> KsResult<String> {
    match variable.value() {
        Value::String(string) => Ok(string.clone()),
        Value::Integer(int) => Ok(int.to_string()),
        Value::Float(float) => Ok(float.to_string()),
        Value::Boolean(boolean) => Ok(boolean.to_string()),
        Value::Function(name) => Ok(format!("{}()", name)),
        Value::NativeFunction(name) => Ok(format!("{}()", name)),
        Value::List(references) => {
            let mut buffer = String::new();
            buffer.push('[');
            buffer = collection_to_string(environment, references, buffer)?;
            buffer.push(']');
            Ok(buffer)
        }
        Value::Tuple(references) => {
            let mut buffer = String::new();
            buffer.push('(');
            buffer = collection_to_string(environment, references, buffer)?;
            buffer.push(')');
            Ok(buffer)
        }
        Value::Module(module) => {
            let mut buffer = String::new();
            buffer.push('{');
            buffer = module_to_string(environment, module, buffer)?;
            buffer.push('}');
            Ok(buffer)
        }
        Value::Null => Ok(String::from("null")),
    }
}

pub fn ks_print(environment: &mut Environment, mut args: Vec<Variable>) -> KsResult<Variable> {
    args.reverse();

    while let Some(arg) = args.pop() {
        let out = value_to_string(environment, arg)?;
        print!("{}", out);
    }
    Ok(Variable::null())
}

pub fn ks_println(environment: &mut Environment, args: Vec<Variable>) -> KsResult<Variable> {
    ks_print(environment, args)?;
    println!("");

    Ok(Variable::null())
}

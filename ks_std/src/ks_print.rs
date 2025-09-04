
use core::global::utils::ks_result::KsResult;

use core::vm::variable::Variable;
use core::vm::value::Value;

pub fn ks_print(args: Vec<Variable>) -> KsResult<Variable> {
    for arg in args {
        match arg.value() {
            Value::Boolean(boolean) => 
                print!("{}", boolean),
            Value::Float(float) => 
                print!("{}", float),
            Value::Integer(integer) =>
                print!("{}", integer),
            Value::String(string) =>
                print!("{}", string),
            Value::Null => 
                print!("null"),
            _ => todo!()
        }
    }

    Ok(Variable::null(0))
}

pub fn ks_println(args: Vec<Variable>) -> KsResult<Variable> {    
    ks_print(args)?;
    println!("");

    Ok(Variable::null(0))
}
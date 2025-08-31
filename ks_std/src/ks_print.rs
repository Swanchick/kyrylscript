
use core::global::utils::ks_result::KsResult;
use std::io;

use core::vm::variable::Variable;
use core::vm::value::Value;
use core::native_registry::native_registry::NativeRegistry;

pub fn ks_print(args: Vec<Variable>) -> KsResult<Variable> {
    print!("{:?}", args);

    Ok(Variable::null(0))
}

pub fn ks_println(args: Vec<Variable>) -> KsResult<Variable> {    
    println!("{:?}", args);

    Ok(Variable::null(0))
}
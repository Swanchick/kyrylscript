use std::io;

use core::vm::variable::Variable;
use core::vm::value::Value;
use core::native_registry::native_registry::NativeRegistry;
use core::global::data_type::DataType;


pub fn ks_range(args: Vec<Variable>) -> io::Result<Variable> {
    if args.len() != 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
    }
 
    todo!()
}
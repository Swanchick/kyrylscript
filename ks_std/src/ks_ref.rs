use std::io;

use core::vm::variable::Variable;
use core::vm::value::Value;

pub fn ks_ref(args: Vec<Variable>) -> io::Result<Variable> {
    if args.len() > 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!")); 
    }

    todo!()
}

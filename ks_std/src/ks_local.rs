use std::io;

use core::vm::variable::Variable;
use core::vm::value::Value;
use core::native_registry::native_registry::NativeRegistry;

pub fn ks_local(args: Vec<Variable>) -> io::Result<Variable> {
    if args.len() > 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
    }

    todo!()
}
use ks_core::kyryl_script::KyrylScript;
use ks_core::parser::data_type::DataType;
use ks_vm::native::native_buffer::NativeBuffer;
use ks_vm::native::native_function::NativeFunction;
use ks_vm::native::native_registry::NativeRegistry;

mod ks_debug;
mod ks_len;
mod ks_print;
mod ks_range;
mod ks_ref;

use ks_debug::ks_debug;
use ks_len::ks_len;
use ks_print::{ks_print, ks_println};
use ks_range::ks_range;
use ks_ref::ks_ref;

pub fn ks_register_std(kyryl_script: &mut KyrylScript) {
    let mut buffer = NativeBuffer::new();

    buffer.add_function("print", NativeFunction::from(ks_print));
    kyryl_script.compiler_mut().register_native("print", 0);
    kyryl_script.parser_mut().register_variable(
        "print",
        DataType::RustFunction {
            return_type: Box::new(DataType::void()),
        },
        true,
    );

    buffer.add_function("println", NativeFunction::from(ks_println));
    kyryl_script.compiler_mut().register_native("println", 1);
    kyryl_script.parser_mut().register_variable(
        "println",
        DataType::RustFunction {
            return_type: Box::new(DataType::void()),
        },
        true,
    );

    buffer.add_function("len", NativeFunction::from(ks_len));
    kyryl_script.compiler_mut().register_native("len", 2);
    kyryl_script.parser_mut().register_variable(
        "len",
        DataType::RustFunction {
            return_type: Box::new(DataType::Int),
        },
        true,
    );

    buffer.add_function("range", NativeFunction::from(ks_range));
    kyryl_script.compiler_mut().register_native("range", 3);
    kyryl_script.parser_mut().register_variable(
        "range",
        DataType::RustFunction {
            return_type: Box::new(DataType::List(Box::new(DataType::Int))),
        },
        true,
    );

    buffer.add_function("ref", NativeFunction::from(ks_ref));
    kyryl_script.compiler_mut().register_native("ref", 4);
    kyryl_script.parser_mut().register_variable(
        "ref",
        DataType::RustFunction {
            return_type: Box::new(DataType::Int),
        },
        true,
    );

    buffer.add_function("debug", NativeFunction::from(ks_debug));
    kyryl_script.compiler_mut().register_native("debug", 5);
    kyryl_script.parser_mut().register_variable(
        "debug",
        DataType::RustFunction {
            return_type: Box::new(DataType::void()),
        },
        true,
    );

    let registry = NativeRegistry::get();
    let mut registry = registry.borrow_mut();
    registry.add_buffer(buffer);
}

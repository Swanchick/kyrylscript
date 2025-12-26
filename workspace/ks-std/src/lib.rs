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

pub fn ks_register_std() {
    let mut buffer = NativeBuffer::new();

    buffer.add_function("print", NativeFunction::from(ks_print));
    buffer.add_function("println", NativeFunction::from(ks_println));
    buffer.add_function("len", NativeFunction::from(ks_len));
    buffer.add_function("range", NativeFunction::from(ks_range));
    buffer.add_function("ref", NativeFunction::from(ks_ref));
    buffer.add_function("debug", NativeFunction::from(ks_debug));

    let registry = NativeRegistry::get();
    let mut registry = registry.borrow_mut();
    registry.add_buffer(buffer);
}

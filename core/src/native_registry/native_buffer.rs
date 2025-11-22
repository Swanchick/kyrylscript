use std::collections::HashMap;

use super::native_function::NativeFunction;
use super::native_types::NativeTypes;

pub struct NativeBuffer {
    natives: HashMap<String, NativeTypes>,
}

impl NativeBuffer {
    pub fn new() -> NativeBuffer {
        NativeBuffer {
            natives: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, native: NativeTypes) {
        self.natives.insert(name.to_string(), native);
    }

    pub fn add_function(&mut self, name: &str, native_function: NativeFunction) {
        self.add(name, NativeTypes::Function(native_function));
    }

    pub fn get_table(&self) -> &HashMap<String, NativeTypes> {
        &self.natives
    }
}

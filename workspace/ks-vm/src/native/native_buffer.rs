use std::collections::HashMap;

use super::native_function::NativeFunction;
use super::native_types::NativeType;

pub struct NativeBuffer {
    natives: HashMap<String, NativeType>,
}

impl NativeBuffer {
    pub fn new() -> NativeBuffer {
        NativeBuffer {
            natives: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, native: NativeType) {
        self.natives.insert(name.to_string(), native);
    }

    pub fn add_function(&mut self, name: &str, native_function: NativeFunction) {
        self.add(name, NativeType::Function(native_function));
    }

    pub fn get_table(&self) -> &HashMap<String, NativeType> {
        &self.natives
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::native_buffer::NativeBuffer;
use super::native_types::NativeType;

thread_local! {
    static NATIVE_REGISTRY: Rc<RefCell<NativeRegistry>> = NativeRegistry::new();
}

pub struct NativeRegistry {
    natives: HashMap<String, NativeType>,
}

impl NativeRegistry {
    pub fn get() -> Rc<RefCell<NativeRegistry>> {
        NATIVE_REGISTRY.with(|registry| registry.clone())
    }

    pub fn new() -> Rc<RefCell<NativeRegistry>> {
        Rc::new(RefCell::new(NativeRegistry {
            natives: HashMap::new(),
        }))
    }

    pub fn add_buffer(&mut self, buffer: NativeBuffer) {
        for (name, native) in buffer.get_table() {
            self.natives.insert(name.to_owned(), native.clone());
        }
    }

    pub fn get_natives(&self) -> &HashMap<String, NativeType> {
        &self.natives
    }

    pub fn get_native(&self, name: &str) -> Option<&NativeType> {
        self.natives.get(name)
    }
}

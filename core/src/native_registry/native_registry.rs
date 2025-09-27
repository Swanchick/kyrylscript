use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use super::native_buffer::NativeBuffer;

use super::native_types::NativeTypes;

thread_local! {
    static NATIVE_REGISTRY: Rc<RefCell<NativeRegistry>> = NativeRegistry::new();
}

pub struct NativeRegistry {
    natives: HashMap<String, NativeTypes>
}

impl NativeRegistry {
    pub fn get() -> Rc<RefCell<NativeRegistry>> {
        NATIVE_REGISTRY.with(|registry| registry.clone())
    }

    pub fn new() -> Rc<RefCell<NativeRegistry>> {
        Rc::new(RefCell::new(
            NativeRegistry { 
                natives: HashMap::new()
            }
        ))
    }

    pub fn add_buffer(&mut self, buffer: NativeBuffer) {
        for (name, native) in buffer.get_table() {
            self.natives.insert(name.to_owned(), native.clone());
        }
    }

    pub fn get_natives(&self) -> &HashMap<String, NativeTypes> {
        &self.natives
    }

    pub fn get_native(&self, name: &str) -> Option<&NativeTypes> {
        self.natives.get(name)
    }

}

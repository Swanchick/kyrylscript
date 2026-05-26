use super::types::Pointer;

#[derive(Debug)]
pub struct CallStack {
    pub return_pointer: Pointer,
    pub stack_pointer: Pointer,
}

impl CallStack {
    pub fn new(return_pointer: Pointer, stack_pointer: Pointer) -> Self {
        Self {
            return_pointer,
            stack_pointer,
        }
    }
}

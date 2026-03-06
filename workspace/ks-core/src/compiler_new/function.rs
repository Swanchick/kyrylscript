use super::types::{Depth, Pointer};

pub struct Function {
    pub pointer: Pointer,
    pub depth: Depth,
}

impl Function {
    pub fn new(pointer: Pointer, depth: Depth) -> Self {
        Self { pointer, depth }
    }
}

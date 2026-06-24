use crate::types::{Arguments, NativeId};

#[derive(Debug, PartialEq)]
pub struct NativeCall {
    pub native_id: NativeId,
    pub arguments: Arguments,
}

impl NativeCall {
    pub fn new(native_id: NativeId, arguments: Arguments) -> Self {
        Self {
            native_id,
            arguments,
        }
    }
}

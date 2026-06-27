use crate::types::{Arguments, NativeId};

#[derive(Debug, PartialEq)]
pub struct NativeCall {
    pub native_id: NativeId,
    pub arguments: Arguments,
    pub runner_id: usize,
}

impl NativeCall {
    pub fn new(native_id: NativeId, arguments: Arguments, runner_id: usize) -> Self {
        Self {
            native_id,
            arguments,
            runner_id,
        }
    }
}

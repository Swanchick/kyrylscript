#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::{GVS, NativeCall};

pub struct VMHelper<'a> {
    pub instruction: u8,
    pub gvs: &'a mut GVS,
    pub native_stack: &'a mut Vec<NativeCall>,
    pub instructions: &'a [u8],
    pub runner_id: usize,
}

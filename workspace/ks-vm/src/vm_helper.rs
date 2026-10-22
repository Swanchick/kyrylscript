use crate::{GVS, NativeCall};

pub struct VMHelper<'a> {
    pub gvs: &'a mut GVS,
    pub native_call: &'a mut Option<NativeCall>,
    pub instructions: &'a [u8],
    pub runner_id: usize,
}

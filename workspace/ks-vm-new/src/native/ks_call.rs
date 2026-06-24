use ks_global::utils::ks_result::KsResult;

use crate::NativeHelper;

pub trait KsCall<'a> {
    fn call(&mut self, arguments: usize, helper: NativeHelper<'a>) -> KsResult<()>;
}

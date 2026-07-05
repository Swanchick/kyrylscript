use crate::NativeHelper;

pub trait KsCall {
    fn call<'a>(&mut self, arguments: usize, helper: NativeHelper<'a>) -> KsResult<()>;
}

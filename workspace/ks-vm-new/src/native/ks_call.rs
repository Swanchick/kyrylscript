use crate::NativeHelper;

use crate::VMResult;

pub trait KsCall {
    fn call<'a>(&mut self, arguments: usize, helper: NativeHelper<'a>) -> VMResult<()>;
}

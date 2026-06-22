use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::NativeHelper;
use crate::types::Arguments;

type NativeFunction<'a> = fn(Arguments, NativeHelper<'a>) -> KsResult<()>;

pub struct NativeRegistry<'a> {
    pub functions: Vec<NativeFunction<'a>>,
}

impl<'a> NativeRegistry<'a> {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn call(
        &self,
        index: usize,
        arguments: Arguments,
        helper: NativeHelper<'a>,
    ) -> KsResult<()> {
        let function = self.functions.get(index).ok_or(KsError::runtime(&format!(
            "Cannot find function with index {}",
            index
        )))?;

        function(arguments, helper)?;

        Ok(())
    }
}

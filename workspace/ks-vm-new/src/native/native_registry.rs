use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::types::Arguments;
use crate::{KsCall, NativeHelper};

pub struct NativeRegistry<'a> {
    pub functions: Vec<Box<dyn KsCall<'a> + 'a>>,
}

impl<'a> NativeRegistry<'a> {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn call(
        &mut self,
        index: usize,
        arguments: Arguments,
        helper: NativeHelper<'a>,
    ) -> KsResult<()> {
        let function = self
            .functions
            .get_mut(index)
            .ok_or(KsError::runtime(&format!(
                "Cannot find function with index {}",
                index
            )))?;

        function.call(arguments, helper)?;

        Ok(())
    }
}

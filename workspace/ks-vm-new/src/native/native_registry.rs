use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use super::{KsCall, NativeHelper};
use crate::types::Arguments;
use crate::{GVS, Runner};

pub struct NativeRegistry {
    pub functions: Vec<Box<dyn KsCall>>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn call(
        &mut self,
        index: usize,
        arguments: Arguments,
        runner: &mut Runner,
        gvs: &mut GVS,
    ) -> KsResult<()> {
        let function = self
            .functions
            .get_mut(index)
            .ok_or(KsError::runtime(&format!(
                "Cannot find function with index {}",
                index
            )))?;

        let helper = NativeHelper { runner, gvs };

        function.call(arguments, helper)?;

        Ok(())
    }
}
